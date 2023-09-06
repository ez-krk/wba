use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct User {
    pub owner: Pubkey,
    pub reputation: u64,
    pub feedbacks: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl User {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner.
        + 8 // reputation.
        + 8 // feedbacks.
        + TIMESTAMP_LENGTH // created_at.
        + BUMP_LENGTH; // bump.
}
