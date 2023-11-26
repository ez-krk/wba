use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use solana_program::blake3;

use solana_program::sysvar::instructions::load_instruction_at_checked;

use std::collections::BTreeMap;

use crate::helpers::ed25519_verify::{ed25519_verify_ix, InstructionSysvar};
use crate::{errors::DiceError, state::Bet};

pub const HOUSE_EDGE: u16 = 150;

#[derive(Accounts)]
#[instruction(seed: u128)]
pub struct ResolveBet<'info> {
    #[account(mut)]
    house: Signer<'info>,
    player: UncheckedAccount<'info>,
    #[account(
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        mut,
        close = player,
        seeds = [b"bet", vault.key().as_ref(), bet.seed.to_le_bytes().as_ref()],
        bump = bet.bump
    )]
    bet: Account<'info, Bet>,
    pub instruction_sysvar: Program<'info, InstructionSysvar>,
    system_program: Program<'info, System>,
}

impl<'info> ResolveBet<'info> {
    pub fn verify_ed25519_signature(&mut self, sig: &[u8]) -> Result<()> {
        // verify the signature
        let ix = load_instruction_at_checked(0, &self.instruction_sysvar.to_account_info())?;
        let pubkey = self.house.key().to_bytes();
        let message = self.bet.to_slice();
        ed25519_verify_ix(&ix, &pubkey, &message, sig);
        Ok(())
    }

    pub fn resolve_bet(&mut self, bumps: &BTreeMap<String, u8>, sig: &[u8]) -> Result<()> {
        let hash = blake3::hash(sig).to_bytes();

        let mut hash_16: [u8; 16] = [0; 16];
        hash_16.copy_from_slice(&hash[0..16]);
        let lower = u128::from_le_bytes(hash_16);
        hash_16.copy_from_slice(&hash[16..32]);
        let upper = u128::from_le_bytes(hash_16);

        let roll = lower.wrapping_add(upper).wrapping_rem(100) as u8 + 1;

        if self.bet.roll < roll {
            let payout = (self.bet.amount as u128)
                .checked_mul(10000 - HOUSE_EDGE as u128)
                .ok_or(DiceError::Overflow)?
                .checked_div(self.bet.roll as u128 - 1)
                .ok_or(DiceError::Overflow)?
                .checked_div(100)
                .ok_or(DiceError::Overflow)? as u64;

            let accounts = Transfer {
                from: self.vault.to_account_info(),
                to: self.player.to_account_info(),
            };

            let seeds = &[
                b"vault",
                &self.house.key().to_bytes()[..],
                &[*bumps.get("vault").ok_or(DiceError::BumpError)?],
            ];

            let signer_seeds = &[&seeds[..]];

            let cpi_ctx = CpiContext::new_with_signer(
                self.system_program.to_account_info(),
                accounts,
                signer_seeds,
            );

            transfer(cpi_ctx, payout)?;
        }
        // emit!(
        //     Resolved {
        //         seed: self.bet.seed,
        //         user: self.player.key(),
        //         roll: self.bet.roll,
        //         result: roll,
        //         bet: self.bet.amount,
        //         payout
        //     }
        // );
        Ok(())
    }
}

// #[event]
// pub struct Resolved {
//     seed: u128,
//     user: Pubkey,
//     roll: u8,
//     result: u8,
//     bet: u64,
//     payout: u64
// }
