use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

declare_id!("SMBKnshAzPi9WZQEUyRxsm6PYZbyZEmxbxbaoMWNHkA");

#[program]
pub mod sombrero {
    use super::*;

    pub fn register_protocol(
        ctx: Context<RegisterProtocol>,
        name: String,
        percent: u64,
        gpg_pubkey: String,
    ) -> Result<()> {
        ctx.accounts
            .register_protocol(&ctx.bumps, name, percent, gpg_pubkey)
    }

    pub fn new_vulnerability(
        ctx: Context<NewVulnerability>,
        message: String,
        seed: u64,
    ) -> Result<()> {
        ctx.accounts.new_vulnerability(&ctx.bumps, message, seed)
    }

    pub fn register_hacker(ctx: Context<RegisterHacker>, name: String) -> Result<()> {
        ctx.accounts.register_hacker(&ctx.bumps, name)
    }

    pub fn new_doxx_sol_hack(ctx: Context<NewSolHack>, amount: u64, seed: u64) -> Result<()> {
        ctx.accounts.new_doxx_sol_hack(&ctx.bumps, amount, seed)
    }

    pub fn approve_doxx_sol_hack(ctx: Context<ApproveDoxxSolHack>) -> Result<()> {
        ctx.accounts.approve_doxx_sol_hack()
    }

    pub fn approve_anon_sol_hack(ctx: Context<ApproveAnonSolHack>) -> Result<()> {
        ctx.accounts.approve_anon_sol_hack()
    }
}
