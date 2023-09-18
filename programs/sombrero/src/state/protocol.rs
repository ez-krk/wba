use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Protocol {
    pub owner: Pubkey,
    pub sol_vault: Pubkey,
    pub name: String,
    pub percent: u64,
    pub gpg_pubkey: String,
    pub paid : u64,
    pub vulnerabilities: u64,
    pub hacks: u64,
    pub approved: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl Protocol {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 2 // owner
        + STRING_LENGTH_PREFIX 
        + MAX_PROTOCOL_LENGTH
        + 8 * 4 // paid, vulnerabilities, hacks
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH; // bump
}
