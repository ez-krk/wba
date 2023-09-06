use anchor_lang::prelude::*;

declare_id!("FEEDLQWp85GJBrum4ASAR6WEAW595oQfaST1N84HZFbS");

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

#[program]
pub mod feedback {
    use super::*;

    pub fn new_session(
        ctx: Context<NewSession>,
        name: String,
        website: String,
        description: String,
    ) -> Result<()> {
        ctx.accounts
            .new_session(&ctx.bumps, name, website, description)
    }

    pub fn new_feedback(
        ctx: Context<NewFeedback>,
        title: String,
        content: String,
        seed: u64,
    ) -> Result<()> {
        ctx.accounts.new_feedback(&ctx.bumps, title, content, seed)
    }

    pub fn new_user(ctx: Context<NewUser>) -> Result<()> {
        ctx.accounts.new_user(&ctx.bumps)
    }
}
