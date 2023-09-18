use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Hack, Hacker, Protocol},
};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(amount: u64, seed: u64)]
pub struct NewHack<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account()]
    pub payout: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"protocol", protocol.owner.as_ref()],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(
        seeds = [b"sol_vault", protocol.sol_vault.as_ref()],
        bump
    )]
    sol_vault: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"hacker", hacker.name.as_bytes().as_ref()],
        bump,
    )]
    pub hacker: Account<'info, Hacker>,
    #[account(
        init,
        payer = signer,
        seeds = [b"hack", protocol.key().as_ref(), hacker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = Hack::LEN
    )]
    pub hack: Account<'info, Hack>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewHack<'info> {
    pub fn new_hack(&mut self, bumps: &BTreeMap<String, u8>, amount: u64, seed: u64) -> Result<()> {
        // pub payout: Pubkey,
        // pub protocol: Pubkey,
        // pub hacker: Pubkey,
        // pub reviewed: bool,
        // pub created_at: i64,
        // pub bump: u8,
        // pub seed: u64,

        let hack = &mut self.hack;

        hack.payout = self.payout.key();
        hack.protocol = self.protocol.key();
        hack.bump = *bumps.get("vulnerability").unwrap();
        hack.created_at = Clock::get().unwrap().unix_timestamp;
        hack.seed = seed;

        let protocol = &mut self.protocol;

        protocol.hacks += 1;

        let hacker = &mut self.hacker;
        hacker.hacks += 1;

        let accounts = Transfer {
            from: self.signer.to_account_info(),
            to: self.sol_vault.to_account_info(),
        };

        let cpi = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(cpi, amount)
    }
}
