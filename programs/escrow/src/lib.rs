use anchor_lang::prelude::*;

declare_id!("KRKA3ugDP5iE79zDuLQEs6qCvY7eSRYbS94vgsTgRU2");

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

#[program]
pub mod escrow {

    use super::*;

    pub fn make(
        ctx: Context<Make>,
        seed: u64,
        deposit_amount: u64,
        offer_amount: u64,
    ) -> Result<()> {
        ctx.accounts.init(&ctx.bumps, seed, offer_amount)?;
        ctx.accounts.transfer_to_vault(deposit_amount)
    }

    pub fn take(ctx: Context<Take>) -> Result<()> {
        ctx.accounts.deposit_to_maker()?;
        ctx.accounts.empty_vault_to_taker()?;
        ctx.accounts.close_vault()
    }

    pub fn update(ctx: Context<Update>, offer_amount: u64) -> Result<()> {
        ctx.accounts.update(offer_amount)
    }

    pub fn refund(ctx: Context<Refund>) -> Result<()> {
        ctx.accounts.empty_vault()?;
        ctx.accounts.close_vault()
    }
}
