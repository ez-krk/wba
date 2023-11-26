use crate::state::{DoxxSolHack, Hacker, Protocol};
use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

#[derive(Accounts)]
pub struct ApproveDoxxSolHack<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        seeds = [b"auth", protocol.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,
    #[account(
        seeds = [b"sol_vault", protocol.key().as_ref()],
        bump
    )]
    sol_vault: SystemAccount<'info>,
    #[account(
        has_one = owner,
        seeds = [b"protocol", owner.key().as_ref()],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(
        seeds = [b"hacker", hacker.name.as_bytes().as_ref()],
        bump,
    )]
    pub hacker: Account<'info, Hacker>,
    #[account(mut)]
    pub payout: SystemAccount<'info>,
    #[account(
        has_one = protocol,
        seeds = [b"doxx", protocol.key().as_ref(), hacker.key().as_ref(), doxx_sol_hack.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub doxx_sol_hack: Account<'info, DoxxSolHack>,
    pub system_program: Program<'info, System>,
}

impl<'info> ApproveDoxxSolHack<'info> {
    pub fn approve_doxx_sol_hack(&mut self) -> Result<()> {
        let sol_hack = &mut self.doxx_sol_hack;

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

        // add amount paid to hacker pda :
        // pub name: String,
        // pub paid : u64,
        // pub vulnerabilities: u64,
        // pub hacks: u64,
        // pub created_at: i64,
        // pub bump: u8,

        let hacker = &mut self.hacker;
        hacker.paid += amount;

        // add amount paid to protocol stats
        protocol.paid += amount;

        // cpi ctx
        let accounts = Transfer {
            from: self.sol_vault.to_account_info(),
            to: self.owner.to_account_info(),
        };
        let seeds = &[
            b"sol_vault",
            self.protocol.to_account_info().key.as_ref(),
            &[self.protocol.vault_bump],
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
