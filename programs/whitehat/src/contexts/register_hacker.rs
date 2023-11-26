use crate::{constants::*, errors::ErrorCode, state::Hacker};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct RegisterHacker<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        seeds = [b"hacker", name.as_bytes().as_ref()],
        bump,
        space = Hacker::LEN
    )]
    pub hacker: Account<'info, Hacker>,
    pub system_program: Program<'info, System>,
}

impl<'info> RegisterHacker<'info> {
    pub fn register_hacker(&mut self, bumps: &BTreeMap<String, u8>, name: String) -> Result<()> {
        if name.len() > MAX_PROTOCOL_LENGTH {
            return err!(ErrorCode::ProtocolNameTooLong);
        } else if name.len() == 0 {
            return err!(ErrorCode::ProtocolNameEmpty);
        }

        let hacker = &mut self.hacker;

        // pub name: String,
        // pub paid : u64,
        // pub vulnerabilities: u64,
        // pub hacks: u64,
        // pub created_at: i64,
        // pub bump: u8,

        hacker.name = name;
        hacker.paid = 0;
        hacker.vulnerabilities = 0;
        hacker.hacks = 0;
        hacker.created_at = Clock::get().unwrap().unix_timestamp;
        hacker.bump = *bumps.get("hacker").unwrap();

        Ok(())
    }
}
