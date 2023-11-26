use anchor_lang::prelude::*;

mod contexts;
mod errors;
mod helpers;
mod state;

use contexts::*;

declare_id!("VG58thf3atpUtDia6NUiJJeJLuHMy2z5CRiTBPXLzwT");

#[program]
pub mod dice {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, amount: u64) -> Result<()> {
        ctx.accounts.init(amount)
    }

    pub fn place_bet(ctx: Context<PlaceBet>, seed: u128, roll: u8, bet: u64) -> Result<()> {
        ctx.accounts.create_bet(&ctx.bumps, seed, roll, bet);
        ctx.accounts.deposit(bet)
    }

    pub fn resolve_bet(ctx: Context<ResolveBet>, sig: Vec<u8>) -> Result<()> {
        ctx.accounts.verify_ed25519_signature(sig);
        ctx.accounts.resolve_bet(&ctx.bumps, sig)
    }
}
