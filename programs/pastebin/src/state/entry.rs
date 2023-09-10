use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Entry {
    pub owner: Pubkey,
    pub entry: String,
    pub created_at: i64,
    pub seed: u64,
    pub bump: u8,
}

impl Entry {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner
        + STRING_LENGTH_PREFIX
        + TIMESTAMP_LENGTH // created_at
        + 8 // seed
        + BUMP_LENGTH; // bump
}
