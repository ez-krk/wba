use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Feedback {
    pub user: Pubkey,
    pub session: Pubkey,
    pub title: String,
    pub content: String,
    pub helpful: bool,
    pub created_at: i64,
    pub seed: u64,
    pub bump: u8,
}

impl Feedback {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner.
        + PUBLIC_KEY_LENGTH // session.
        + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH // title.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH // content.
        + BOOL_LENGTH // helpful.
        + TIMESTAMP_LENGTH // created_at.
        + 8 // seed.
        + BUMP_LENGTH; // bump.
}
