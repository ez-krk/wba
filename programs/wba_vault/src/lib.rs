use anchor_lang::prelude::*;

mod contexts;
mod state;

use contexts::*;

declare_id!("VG58thf3atpUtDia6NUiJJeJLuHMy2z5CRiTBPXLzwT");

#[program]
pub mod wba_vault {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.init(&ctx.bumps)
    }

    pub fn deposit(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)
    }

    pub fn withdraw(ctx: Context<Payment>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw(amount)
    }

    pub fn deposit_spl(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        ctx.accounts.deposit_spl(amount)
    }

    pub fn withdraw_spl(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        ctx.accounts.withdraw_spl(amount)
    }
}
