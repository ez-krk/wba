use crate::state::{AnonSolHack, Protocol, Vault};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct ApproveAnonSolHack<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        seeds = [b"auth", state.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,
    #[account(
        seeds = [b"sol_vault", state.key().as_ref()],
        bump
    )]
    sol_vault: SystemAccount<'info>,
    #[account(
        seeds = [b"state", owner.key().as_ref()],
        bump
    )]
    state: Account<'info, Vault>,
    #[account(
        has_one = owner,
        seeds = [b"protocol", owner.key().as_ref()],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(mut)]
    pub payout: AccountInfo<'info>,
    #[account(
        has_one = protocol,
        seeds = [b"sol_hack", protocol.key().as_ref(), protocol.vulnerabilities.to_le_bytes().as_ref(), anon_sol_hack.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub anon_sol_hack: Account<'info, AnonSolHack>,
    pub system_program: Program<'info, System>,
}

impl<'info> ApproveAnonSolHack<'info> {
    pub fn approve_anon_sol_hack(&mut self) -> Result<()> {
        let sol_hack = &mut self.anon_sol_hack;

        let protocol = &mut self.protocol;

        // pub owner: Pubkey,
        // pub sol_vault: Pubkey,
        // pub name: String,
        // pub percent: u8,
        // pub gpg_pubkey: String,
        // pub paid : u64,
        // pub vulnerabilities: u64,
        // pub hacks: u64,
        // pub approved: u64,
        // pub created_at: i64,
        // pub bump: u8,

        let amount = protocol.percent * sol_hack.value / 100;
        protocol.paid += amount;

        let accounts = Transfer {
            from: self.sol_vault.to_account_info(),
            to: self.owner.to_account_info(),
        };

        let seeds = &[
            b"sol_vault",
            self.state.to_account_info().key.as_ref(),
            &[self.state.vault_bump],
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx_cpi = CpiContext::new_with_signer(
            self.system_program.to_account_info(),
            accounts,
            signer_seeds,
        );

        transfer(ctx_cpi, amount)
    }
}
