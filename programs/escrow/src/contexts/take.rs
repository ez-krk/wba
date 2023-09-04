use crate::state::Escrow;
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{
        close_account, transfer as spl_transfer, CloseAccount, Mint, Token, TokenAccount,
        Transfer as SplTransfer,
    },
};

#[derive(Accounts)]
pub struct Take<'info> {
    /// CHECK: no need to check this.
    pub maker: UncheckedAccount<'info>,
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_token,
        associated_token::authority = maker
    )]
    pub maker_receive_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = maker_token,
        associated_token::authority = taker
    )]
    pub taker_ata: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_token,
        associated_token::authority = taker
    )]
    pub taker_receive_ata: Account<'info, TokenAccount>,
    pub maker_token: Box<Account<'info, Mint>>,
    pub taker_token: Box<Account<'info, Mint>>,
    #[account(
        seeds = [b"auth", escrow.key().as_ref()],
        bump = escrow.auth_bump
    )]
    /// CHECK: This is not dangerous because this account doesn't exist
    pub auth: UncheckedAccount<'info>,
    #[account(
        mut,
        seeds = [b"vault", escrow.key().as_ref()],
        bump = escrow.vault_bump,
        token::mint = maker_token,
        token::authority = auth
    )]
    pub vault: Account<'info, TokenAccount>,
    #[account(
        has_one = maker,
        has_one = taker_token,
        has_one = maker_token,
        seeds = [b"escrow", maker.key.as_ref(), escrow.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub escrow: Box<Account<'info, Escrow>>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

impl<'info> Take<'info> {
    pub fn deposit_to_maker(&self) -> Result<()> {
        let cpi_accounts = SplTransfer {
            from: self.taker_ata.to_account_info(),
            to: self.maker_receive_ata.to_account_info(),
            authority: self.taker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);

        spl_transfer(cpi_ctx, self.escrow.offer_amount)
    }

    pub fn empty_vault_to_taker(&self) -> Result<()> {
        let signer_seeds = &[&b"auth"[..], &[self.escrow.auth_bump]];

        let seeds = [&signer_seeds[..]];

        let cpi_accounts = SplTransfer {
            from: self.vault.to_account_info(),
            to: self.taker_receive_ata.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, &seeds);

        spl_transfer(cpi_ctx, self.vault.amount)
    }

    pub fn close_vault(&self) -> Result<()> {
        let signer_seeds = &[&b"auth"[..], &[self.escrow.auth_bump]];

        let seeds = [&signer_seeds[..]];

        let cpi_accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.taker.to_account_info(),
            authority: self.auth.to_account_info(),
        };

        let cpi_ctx =
            CpiContext::new_with_signer(self.token_program.to_account_info(), cpi_accounts, &seeds);

        close_account(cpi_ctx)
    }
}
