use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Questions {
    pub owner: Pubkey,
    pub questions: Vec<String>,
    pub participants: u64,
    pub bump: u8,
}

impl Questions {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner
        + VECTOR_LENGTH_PREFIX + ((STRING_LENGTH_PREFIX + MAX_NAME_LENGTH) * 3) // questions, max 3
        + BUMP_LENGTH; // bump
}
