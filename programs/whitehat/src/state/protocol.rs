use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Protocol {
    pub owner: Pubkey,
    pub sol_vault: Pubkey,
    pub name: String,
    pub percent: u64,
    pub paid : u64,
    pub vulnerabilities: u64,
    pub hacks: u64,
    pub approved: u64,
    pub created_at: i64,
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Protocol {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 2 // owner
        + STRING_LENGTH_PREFIX 
        + MAX_PROTOCOL_LENGTH
        + 8 * 5 // percent, paid, vulnerabilities, hacks, approved
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH * 3; // bump
}
