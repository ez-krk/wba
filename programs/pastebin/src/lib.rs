use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

declare_id!("KRKTFYKLDDLMtYFLGZhkCLgZcorCmXmNG79BVtfm9Lp");

#[program]
pub mod pastebin {
    use super::*;

    pub fn new_entry(ctx: Context<NewEntry>, input: String, seed: u64) -> Result<()> {
        ctx.accounts.new_entry(&ctx.bumps, input, seed)
    }
}
