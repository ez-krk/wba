use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

declare_id!("CCLnXJAJYFjCHLCugpBCEQKrpiSApiRM4UxkBUHJRrv4");

#[program]
pub mod game {
    use super::*;

    pub fn create_game(
        ctx: Context<Initialize>,
        uri: String,
        name: String,
        symbol: String,
    ) -> Result<()> {
        ctx.accounts.create_mint(&ctx.bumps, uri, name, symbol)?;
        ctx.accounts.init(&ctx.bumps)
    }

    pub fn new_player(ctx: Context<NewPlayer>, x: u8, y: u8) -> Result<()> {
        ctx.accounts.new_player(&ctx.bumps, x, y)
    }

    pub fn attack(ctx: Context<NewAttack>) -> Result<()> {
        unimplemented!()
    }

    pub fn heal(ctx: Context<Heal>) -> Result<()> {
        unimplemented!()
    }
}
