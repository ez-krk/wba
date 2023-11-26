use crate::state::{DoxxSolHack, Hacker, Protocol};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(amount: u64, seed: u64)]
pub struct NewDoxxSolHack<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account()]
    pub payout: SystemAccount<'info>,
    #[account(
        mut,
        seeds = [b"protocol", protocol.owner.as_ref()],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(
        mut,
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
        seeds = [b"hack", protocol.key().as_ref(), signer.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = DoxxSolHack::LEN
    )]
    pub hack: Account<'info, DoxxSolHack>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewDoxxSolHack<'info> {
    pub fn new_doxx_sol_hack(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        amount: u64,
        seed: u64,
    ) -> Result<()> {
        // pub payout: Pubkey,
        // pub protocol: Pubkey,
        // pub hacker: Pubkey,
        // pub value: u64,
        // pub reviewed: bool,
        // pub created_at: i64,
        // pub bump: u8,
        // pub seed: u64,

        let hack = &mut self.hack;

        hack.payout = self.payout.key();
        hack.protocol = self.protocol.key();
        hack.hacker = self.hacker.key();
        hack.value = amount;
        hack.bump = *bumps.get("doxx").unwrap();
        hack.created_at = Clock::get()?.unix_timestamp;
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
