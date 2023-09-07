use crate::state::{Launchpad, User};

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Payment<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        init_if_needed,
        payer = owner,
        seeds = [b"user", launchpad.key().as_ref()],
        space = User::LEN,
        bump
    )]
    user: Account<'info, User>,
    #[account(
        mut,
        seeds = [b"sol_vault", launchpad.key().as_ref()],
        bump = launchpad.sol_vault_bump
    )]
    sol_vault: SystemAccount<'info>,
    #[account(
        seeds = [b"launchpad", owner.key().as_ref()],
        bump = launchpad.launchpad_bump
    )]
    launchpad: Account<'info, Launchpad>,
    system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        let user = &mut self.user;
        user.user = self.owner.key();
        user.launchpad = self.launchpad.key();
        user.deposit += amount;
        let accounts = Transfer {
            from: self.owner.to_account_info(),
            to: self.sol_vault.to_account_info(),
        };

        let cpi = CpiContext::new(self.system_program.to_account_info(), accounts);

        transfer(cpi, amount)
    }

    pub fn withdraw(&mut self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.sol_vault.to_account_info(),
            to: self.owner.to_account_info(),
        };

        let seeds = &[
            b"sol_vault",
            self.launchpad.to_account_info().key.as_ref(),
            &[self.launchpad.sol_vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer(cpi, amount)
    }
}
