use anchor_lang::prelude::*;

use crate::constants::*;

#[account]
pub struct Attack {
    pub player: Pubkey,
    pub target: Pubkey,
    pub started_at: i64,
    pub resolved: bool,
    pub bump: u8,
}

impl Attack {
    pub const LEN: usize = DISCRIMINATOR_LENGTH + (32 * 2) + 1 + 8 + 8;
}
