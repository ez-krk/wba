use crate::{errors::DiceError, state::Bet};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(seed: u128)]
pub struct PlaceBet<'info> {
    #[account(mut)]
    player: Signer<'info>,
    house: UncheckedAccount<'info>,
    #[account(
        seeds = [b"vault", house.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        init,
        payer = player,
        seeds = [b"bet", vault.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = Bet::LEN,
        bump
    )]
    bet: Account<'info, Bet>,
    system_program: Program<'info, System>,
}

impl<'info> PlaceBet<'info> {
    pub fn create_bet(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        seed: u128,
        roll: u8,
        amount: u64,
    ) -> Result<()> {
        self.bet.slot = Clock::get()?.slot;
        self.bet.player = self.player.key();
        self.bet.seed = seed;
        self.bet.roll = roll;
        self.bet.amount = amount;
        self.bet.bump = *bumps.get("bet").unwrap();
        Ok(())
    }

    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.house.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.system_program.to_account_info(), cpi_accounts);

        transfer(cpi_ctx, amount)
    }
}
