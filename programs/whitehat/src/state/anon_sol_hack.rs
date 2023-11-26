use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct AnonSolHack {
    pub payout: Pubkey,
    pub protocol: Pubkey,
    pub value: u64,
    pub reviewed: bool,
    pub created_at: i64,
    pub bump: u8,
    pub seed: u64,
}

impl AnonSolHack {
    pub const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH * 2 // payout, protocol
        + 1 // reviewed
        + TIMESTAMP_LENGTH // created_at
        + BUMP_LENGTH // bump
        + SEED_LENGTH; // seed
}
