use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Answers {
    pub owner: Pubkey,
    pub form: Pubkey,
    pub answers: Vec<String>,
    pub bump: u8,
}

impl Answers {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner
        + PUBLIC_KEY_LENGTH // form
        + VECTOR_LENGTH_PREFIX + ((STRING_LENGTH_PREFIX + MAX_NAME_LENGTH) * 3) // answers, max 3
        + BUMP_LENGTH; // bump
}
