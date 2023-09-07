use anchor_lang::prelude::*;

#[account]
pub struct Attack {
    pub player: Pubkey,
    pub target: Pubkey,
    pub started_at: i64,
    pub bump: u8,
}

impl Attack {
    pub const LEN: usize = 8 + (32 * 2) + 8;
}
