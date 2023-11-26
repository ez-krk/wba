use anchor_lang::prelude::*;

declare_id!("BXctdRq8zm4Zx8dsAMbZ5BU4k1EnuZvv4Jm44d9W4v3Q");

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

#[program]
pub mod mail {
    use super::*;

    pub fn new_inbox(ctx: Context<NewInbox>) -> Result<()> {
        ctx.accounts.new_inbox(&ctx.bumps)
    }

    pub fn new_mail(ctx: Context<NewMail>, message: Vec<u8>, seed: u128) -> Result<()> {
        ctx.accounts.new_mail(&ctx.bumps, message, seed)
    }
}
