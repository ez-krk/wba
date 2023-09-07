use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Questions {
    pub owner: Pubkey,
    pub questions: Vec<String>,
    pub participants: u64,
    pub created_at: i64,
    pub seed: u64,
    pub bump: u8,
}

impl Questions {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner
        + VECTOR_LENGTH_PREFIX
        + 8
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH; // bump
}
