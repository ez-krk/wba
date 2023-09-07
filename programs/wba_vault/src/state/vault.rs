use anchor_lang::prelude::*;

#[account]
pub struct Vault {
    pub auth_bump: u8,
    pub vault_bump: u8,
    pub state_bump: u8,
}

impl Vault {
    pub const LEN: usize = 8 + 3 * 1;
}
