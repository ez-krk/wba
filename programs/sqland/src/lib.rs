use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

declare_id!("CCLnXJAJYFjCHLCugpBCEQKrpiSApiRM4UxkBUHJRrv4");

#[program]
pub mod sqland {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        // uri: String,
        // name: String,
        // symbol: String,
    ) -> Result<()> {
        // ctx.accounts.create_mint(&ctx.bumps, uri, name, symbol)?;
        ctx.accounts.init(&ctx.bumps)
    }

    pub fn new_player(ctx: Context<NewPlayer>, x: u8, y: u8) -> Result<()> {
        ctx.accounts.new_player(&ctx.bumps, x, y)
    }

    pub fn new_attack(ctx: Context<NewAttack>) -> Result<()> {
        ctx.accounts.new_attack(&ctx.bumps)
    }

    pub fn close_attack(ctx: Context<CloseAttack>) -> Result<()> {
        ctx.accounts.close_attack(&ctx.bumps)
    }

    pub fn heal(ctx: Context<Heal>) -> Result<()> {
        ctx.accounts.heal()
    }
}
