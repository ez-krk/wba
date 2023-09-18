use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Protocol, Vulnerability},
};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct Approve<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(mut)]
    pub payout: AccountInfo<'info>,
    #[account(
        mut,
        seeds = [b"protocol", owner.key().as_ref()],
        bump,
    )]
    pub protocol: Account<'info, Protocol>,
    #[account(
        has_one = owner,
        seeds = [b"vulnerability", protocol.key().as_ref(), owner.key().as_ref(), vulnerability.seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub vulnerability: Account<'info, Vulnerability>,
    pub system_program: Program<'info, System>,
}
