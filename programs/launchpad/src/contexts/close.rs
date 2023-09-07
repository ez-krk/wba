use crate::state::Vault;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{transfer, Mint, Token, TokenAccount, Transfer};

use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    owner_ata: Account<'info, TokenAccount>,
    #[account(
        seeds=[b"auth", state.key().as_ref()],
        bump=state.auth_bump
    )]
    /// CHECK : this is safe
    auth: UncheckedAccount<'info>,
    #[account(
        mut,
        token::mint = mint,
        token::authority = auth,
        seeds=[b"spl_vault", state.key().as_ref()],
        bump
    )]
    spl_vault: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds=[b"vault", state.key().as_ref()],
        bump=state.vault_bump
    )]
    vault: SystemAccount<'info>,
    mint: Account<'info, Mint>,
    #[account(
        mut,
        close = owner,
        seeds=[b"state", owner.key().as_ref()],
        bump = state.state_bump
    )]
    state: Account<'info, Vault>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}
