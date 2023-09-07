use crate::state::Launchpad;
use anchor_lang::prelude::*;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(amount: u64, threshold: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    // pda signer /wo acc
    #[account(
        seeds = [b"auth", launchpad.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,
    mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub owner_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"sol_vault", launchpad.key().as_ref()],
        bump
    )]
    sol_vault: SystemAccount<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"spl_vault", launchpad.key().as_ref()],
        token::mint = mint,
        token::authority = spl_vault,
        bump
    )]
    spl_vault: Account<'info, TokenAccount>,
    #[account(
        init,
        payer = owner,
        space = Launchpad::LEN,
        seeds = [b"launchpad", owner.key().as_ref()],
        bump
    )]
    launchpad: Account<'info, Launchpad>,
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
}

impl<'info> Initialize<'info> {
    pub fn init(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        amount: u64,
        threshold: u64,
    ) -> Result<()> {
        let launchpad = &mut self.launchpad;
        launchpad.auth_bump = *bumps.get("auth").unwrap();
        launchpad.sol_vault_bump = *bumps.get("sol_vault").unwrap();
        launchpad.spl_vault_bump = *bumps.get("spl_vault").unwrap();
        launchpad.launchpad_bump = *bumps.get("launchpad").unwrap();
        launchpad.mint = self.mint.key();
        launchpad.amount = amount;
        launchpad.threshold = threshold;
        Ok(())
    }

    pub fn transfer_to_vault(&self, amount: u64) -> Result<()> {
        let cpi_accounts = Transfer {
            from: self.owner_ata.to_account_info(),
            to: self.spl_vault.to_account_info(),
            authority: self.owner.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        transfer(cpi_ctx, amount)
    }
}
