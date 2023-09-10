use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

declare_id!("KRKwxs7rsAaeo6oULCnqdhEYQeXvwW6CX9jrHZSyX9y");

#[program]
pub mod pastebin {
    use super::*;

    pub fn new_entry(ctx: Context<NewEntry>, input: String, seed: u64) -> Result<()> {
        ctx.accounts.new_entry(&ctx.bumps, input, seed)
    }
}
