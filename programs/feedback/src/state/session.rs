use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Session {
    pub owner: Pubkey,
    pub name: String,
    pub website: String,
    pub description: String,
    pub bump: u8,
}

impl Session {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner.
        + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH // name.
        + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH // website.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH // description.
        + BUMP_LENGTH; // bump.
}
