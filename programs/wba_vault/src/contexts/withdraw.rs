use crate::state::Vault;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner,
    )]
    owner_ata: Account<'info, TokenAccount>,
    mint: Account<'info, Mint>,
    #[account(
        seeds = [b"auth", state.key().as_ref()],
        bump = state.auth_bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"spl_vault", state.key().as_ref()],
        token::mint = mint,
        token::authority = auth,
        bump
    )]
    vault: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"state", owner.key().as_ref()],
        bump = state.state_bump
    )]
    state: Account<'info, Vault>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}
impl<'info> Withdraw<'info> {
    pub fn withdraw_spl(&mut self, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.owner_ata.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let seeds = &[
            b"auth",
            self.state.to_account_info().key.as_ref(),
            &[self.state.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            self.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer(cpi, amount)
    }
}
