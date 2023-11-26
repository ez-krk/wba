use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Inbox {
    pub owner: Pubkey,
    pub messages: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl Inbox {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner.
        + 8 // messages
        + 8 // created_at
        + BUMP_LENGTH // bump.
        + STRING_LENGTH_PREFIX;
}
