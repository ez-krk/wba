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

    pub fn register_protocol(ctx: Context<RegisterProtocol>, name: String) -> Result<()> {
        ctx.accounts.register_protocol(&ctx.bumps, name)
    }

    pub fn new_vulnerability(
        ctx: Context<NewVulnerability>,
        title: String,
        form: Vec<String>,
        seed: u64,
    ) -> Result<()> {
        ctx.accounts
            .new_vulnerability(&ctx.bumps, title, form, seed)
    }
}
