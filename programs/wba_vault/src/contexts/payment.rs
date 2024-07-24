use crate::state::Vault;

use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct Payment<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump
    )]
    sol_vault: SystemAccount<'info>,
    #[account(
        seeds = [b"state", owner.key().as_ref()],
        bump = state.state_bump
    )]
    state: Account<'info, Vault>,
    system_program: Program<'info, System>,
}

impl<'info> Payment<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
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
            b"vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
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
