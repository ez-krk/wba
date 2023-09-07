use anchor_lang::prelude::*;

#[account]
pub struct Launchpad {
    pub auth_bump: u8,
    pub sol_vault_bump: u8,
    pub spl_vault_bump: u8,
    pub launchpad_bump: u8,
    pub amount: u64,
    pub threshold: u64,
    pub mint: Pubkey,
    pub created_at: i64,
}

impl Launchpad {
    pub const LEN: usize = 8 + (3 * 4) + (2 * 8) + 32 + 8;
}
