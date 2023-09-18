use crate::{constants::*, errors::ErrorCode, state::Protocol};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct RegisterProtocol<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"protocol", owner.key().as_ref()],
        bump,
        space = Protocol::LEN
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

impl<'info> RegisterProtocol<'info> {
    pub fn register_protocol(&mut self, bumps: &BTreeMap<String, u8>, name: String) -> Result<()> {
        if name.len() > MAX_PROTOCOL_LENGTH {
            return err!(ErrorCode::ProtocolNameTooLong);
        } else if name.len() == 0 {
            return err!(ErrorCode::ProtocolNameEmpty);
        }

        let protocol = &mut self.protocol;
        protocol.bump = *bumps.get("protocol").unwrap();
        protocol.owner = self.owner.key();
        protocol.hackers = 0;
        protocol.vulnerabilities = 0;
        protocol.created_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}
