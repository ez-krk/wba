use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Answers {
    pub owner: Pubkey,
    pub questions: Pubkey,
    pub answers: Vec<String>,
    pub created_at: i64,
    pub bump: u8,
}

impl Answers {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner
        + PUBLIC_KEY_LENGTH // form
        + VECTOR_LENGTH_PREFIX // 3 answers
        + TIMESTAMP_LENGTH
        + BUMP_LENGTH; // bump
}
