use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Protocol {
    pub owner: Pubkey,
    pub name: String,
    pub hackers: u64,
    pub vulnerabilities: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl Protocol {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner
        + STRING_LENGTH_PREFIX 
        + MAX_PROTOCOL_LENGTH
        + 8
        + 8
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH; // bump
}
