use crate::state::Vault;

use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    // pda signer /wo acc
    #[account(
        seeds = [b"auth", state.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,
    // lamports stored here
    #[account(
        seeds = [b"vault", state.key().as_ref()],
        bump
    )]
    vault: SystemAccount<'info>,
    #[account(
        init,
        payer = owner,
        space = Vault::LEN,
        seeds = [b"state", owner.key().as_ref()],
        bump
    )]
    state: Account<'info, Vault>,
    system_program: Program<'info, System>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, bumps: &BTreeMap<String, u8>) -> Result<()> {
        let state = &mut self.state;
        state.auth_bump = *bumps.get("auth").unwrap();
        state.vault_bump = *bumps.get("vault").unwrap();
        state.state_bump = *bumps.get("state").unwrap();
        Ok(())
    }
}
