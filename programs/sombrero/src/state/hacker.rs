use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Hacker {
    pub name: String,
    pub paid : u64,
    pub vulnerabilities: u64,
    pub hacks: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl Hacker {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner
        + STRING_LENGTH_PREFIX 
        + MAX_PROTOCOL_LENGTH
        + 8 * 3 // paid, vulnerabilities, hacks
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH; // bump
}
