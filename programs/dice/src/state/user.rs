use anchor_lang::prelude::*;

#[account]
pub struct User {
    pub user: Pubkey,
    pub launchpad: Pubkey,
    pub deposit: u64,
}

impl User {
    pub const LEN: usize = 8 + (32 * 2) + 8;
}
