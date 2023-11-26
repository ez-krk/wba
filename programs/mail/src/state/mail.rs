use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Mail {
    pub sender: Pubkey,
    pub inbox: Pubkey,
    pub message: Vec<u8>,
    pub created_at: i64,
    pub seed: u128,
    pub bump: u8,
}

impl Mail {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 2 // owner, session
        + STRING_LENGTH_PREFIX
        + TIMESTAMP_LENGTH // created_at.
        + 16 // seed.
        + BUMP_LENGTH; // bump.
}
