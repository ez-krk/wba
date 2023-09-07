use anchor_lang::prelude::*;

#[account]
pub struct Game {
    pub mint: Pubkey,
    pub alive: u64,
    pub dead: u64,
    pub token_minted: u64,
    pub token_burnt: u64,
    pub auth_bump: u8,
    pub sol_vault_bump: u8,
    pub spl_vault_bump: u8,
    pub bump: u8,
}

impl Game {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 8 + 8;
}
