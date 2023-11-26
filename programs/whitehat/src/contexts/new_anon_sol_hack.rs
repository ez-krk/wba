use crate::state::{DoxxSolHack, Protocol};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(amount: u64, seed: u64)]
pub struct NewAnonSolHack<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account()]
    pub payout: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"protocol", protocol.owner.as_ref()],
        bump = protocol.state_bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(
        mut,
        seeds = [b"sol_vault", protocol.sol_vault.as_ref()],
        bump = protocol.vault_bump
    )]
    sol_vault: SystemAccount<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"hack", protocol.key().as_ref(), signer.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = DoxxSolHack::LEN
    )]
    pub hack: Account<'info, DoxxSolHack>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewAnonSolHack<'info> {
    pub fn new_anon_sol_hack(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        amount: u64,
        seed: u64,
    ) -> Result<()> {
        let hack = &mut self.hack;

        // pub payout: Pubkey,
        // pub protocol: Pubkey,
        // pub value: u64,
        // pub reviewed: bool,
        // pub created_at: i64,
        // pub bump: u8,
        // pub seed: u64,


        hack.payout = self.payout.key();
        hack.protocol = self.protocol.key();
        hack.value = amount;
        hack.bump = *bumps.get("hack").unwrap();
        hack.created_at = Clock::get()?.unix_timestamp;
        hack.seed = seed;

        let protocol = &mut self.protocol;

        protocol.hacks += 1;

        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.sol_vault.to_account_info(),
        };

        let cpi = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(cpi, amount)
    }
}
