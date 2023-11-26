use crate::{constants::*, errors::ErrorCode, state::Protocol};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(name: String, percent: u64)]
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
    #[account(
        seeds = [b"auth", protocol.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,
    #[account(
        seeds = [b"sol_vault", protocol.key().as_ref()],
        bump
    )]
    sol_vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> RegisterProtocol<'info> {
    pub fn register_protocol(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        name: String,
        percent: u64,
    ) -> Result<()> {
        if name.len() > MAX_PROTOCOL_LENGTH {
            return err!(ErrorCode::ProtocolNameTooLong);
        } else if name.len() == 0 {
            return err!(ErrorCode::ProtocolNameEmpty);
        }

        // if gpg_pubkey.len() < MIN_GPG_LENGTH {
        //     return err!(ErrorCode::GPGKeyTooSmall);
        // } else if gpg_pubkey.len() > MAX_GPG_LENGTH {
        //     return err!(ErrorCode::GPGKeyTooBig);
        // } else if gpg_pubkey.len() == 0 {
        //     return err!(ErrorCode::GPGKeyEmpty);
        // }

        // pub owner: Pubkey,
        // pub sol_vault: Pubkey,
        // pub name: String,
        // pub percent: u8,
        // pub paid : u64,
        // pub vulnerabilities: u64,
        // pub hacks: u64,
        // pub approved: u64,
        // pub created_at: i64,
        // pub bump: u8,

        let protocol = &mut self.protocol;
        protocol.owner = self.owner.key();
        protocol.name = name;
        protocol.percent = percent;
        protocol.paid = 0;
        protocol.vulnerabilities = 0;
        protocol.hacks = 0;
        protocol.auth_bump = *bumps.get("auth").unwrap();
        protocol.vault_bump = *bumps.get("sol_vault").unwrap();
        protocol.state_bump = *bumps.get("protocol").unwrap();
        protocol.created_at = Clock::get()?.unix_timestamp;

        Ok(())
    }
}
