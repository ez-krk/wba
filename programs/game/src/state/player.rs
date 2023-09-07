use anchor_lang::prelude::*;

#[account]
pub struct Player {
    pub player: Pubkey,
    pub x: u8,
    pub y: u8,
    pub health: u8,
    pub kills: u64,
    pub shield: i64,
    pub idle: bool,
    pub bump: u8,
}

impl Player {
    pub const LEN: usize = 8 + 32 + 1 + 8 + 8 + 1;
}
