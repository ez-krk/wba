use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{
    transfer as spl_transfer, Mint, Token, TokenAccount, Transfer as SplTransfer,
};

declare_id!("KRKATotRqvLrvEjyeGKeBsR5W5UQxe29Ap2kLgwRcC7");

#[program]
pub mod launchpad {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.state.auth_bump = *ctx.bumps.get("auth").unwrap();
        ctx.accounts.state.vault_bump = *ctx.bumps.get("vault").unwrap();
        ctx.accounts.state.state_bump = *ctx.bumps.get("state").unwrap();
        Ok(())
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.owner.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
        };

        let cpi = CpiContext::new(ctx.accounts.system_program.to_account_info(), accounts);

        transfer(cpi, amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        let accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner.to_account_info(),
        };

        let seeds = &[
            b"vault",
            ctx.accounts.state.to_account_info().key.as_ref(),
            &[ctx.accounts.state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer(cpi, amount)
    }

    pub fn deposit_spl(ctx: Context<DepositSPL>, amount: u64) -> Result<()> {
        let accounts = SplTransfer {
            from: ctx.accounts.owner_ata.to_account_info(),
            to: ctx.accounts.vault.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };

        let cpi = CpiContext::new(ctx.accounts.token_program.to_account_info(), accounts);

        spl_transfer(cpi, amount)
    }

    pub fn withdraw_spl(ctx: Context<WithdrawSPL>, amount: u64) -> Result<()> {
        let accounts = SplTransfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.owner_ata.to_account_info(),
            authority: ctx.accounts.auth.to_account_info(),
        };

        let seeds = &[
            b"auth",
            ctx.accounts.state.to_account_info().key.as_ref(),
            &[ctx.accounts.state.auth_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let cpi = CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        spl_transfer(cpi, amount)
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    // pda signer /wo acc
    #[account(
        seeds = [b"auth", state.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,
    // lamports stored here
    #[account(
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        init,
        payer = owner,
        space = VaultState::LEN,
        seeds = [b"state", owner.key().as_ref()],
        bump
    )]
    state: Account<'info, VaultState>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Payment<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"vault", state.key().as_ref()],
        bump = state.vault_bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        seeds = [b"state", owner.key().as_ref()],
        bump = state.state_bump
    )]
    state: Account<'info, VaultState>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositSPL<'info> {
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
        init,
        payer = owner,
        seeds = [b"spl_vault", state.key().as_ref()],
        token::mint = mint,
        token::authority = vault,
        bump
    )]
    vault: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"state", owner.key().as_ref()],
        bump = state.state_bump
    )]
    state: Account<'info, VaultState>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSPL<'info> {
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
    state: Account<'info, VaultState>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CloseVault<'info> {
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
    state: Account<'info, VaultState>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Participant {
    auth_bump: u8,
    vault_bump: u8,
    state_bump: u8,
}

#[account]
pub struct VaultState {
    auth_bump: u8,
    vault_bump: u8,
    state_bump: u8,
}

impl VaultState {
    const LEN: usize = 8 + 3 * 1;
}
