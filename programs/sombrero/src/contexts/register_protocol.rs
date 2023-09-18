use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Protocol, Vault},
};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(name: String, gpg_pubkey: String)]
pub struct RegisterProtocol<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        seeds = [b"auth", state.key().as_ref()],
        bump
    )]
    /// CHECK: This is safe
    auth: UncheckedAccount<'info>,
    // lamports stored here
    #[account(
        seeds = [b"sol_vault", state.key().as_ref()],
        bump
    )]
    sol_vault: SystemAccount<'info>,
    #[account(
        init,
        payer = owner,
        space = Vault::LEN,
        seeds = [b"state", owner.key().as_ref()],
        bump
    )]
    state: Account<'info, Vault>,
    #[account(
        init,
        payer = owner,
        seeds = [b"protocol", owner.key().as_ref()],
        bump,
        space = Protocol::LEN + (STRING_LENGTH_PREFIX + gpg_pubkey.len())
    )]
    pub protocol: Account<'info, Protocol>,
    pub system_program: Program<'info, System>,
}

impl<'info> RegisterProtocol<'info> {
    pub fn register_protocol(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        name: String,
        percent: u8,
        gpg_pubkey: String,
    ) -> Result<()> {
        if name.len() > MAX_PROTOCOL_LENGTH {
            return err!(ErrorCode::ProtocolNameTooLong);
        } else if name.len() == 0 {
            return err!(ErrorCode::ProtocolNameEmpty);
        }

        if gpg_pubkey.len() < MIN_GPG_LENGTH {
            return err!(ErrorCode::GPGKeyTooSmall);
        } else if gpg_pubkey.len() > MAX_GPG_LENGTH {
            return err!(ErrorCode::GPGKeyTooBig);
        } else if gpg_pubkey.len() == 0 {
            return err!(ErrorCode::GPGKeyEmpty);
        }

        // pub owner: Pubkey,
        // pub sol_vault: Pubkey,
        // pub name: String,
        // pub percent: u8,
        // pub gpg_pubkey: String,
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
        protocol.gpg_pubkey = gpg_pubkey;
        protocol.bump = *bumps.get("protocol").unwrap();
        protocol.created_at = Clock::get().unwrap().unix_timestamp;

        // pub protocol: Pubkey,
        // pub total_deposit: u64,
        // pub total_paid: u64,
        // pub sol_vault: Pubkey,
        // pub auth_bump: u8,
        // pub vault_bump: u8,
        // pub state_bump: u8,

        let state = &mut self.state;

        state.protocol = protocol.key();
        state.deposit = 0;
        state.paid = 0;
        state.sol_vault = self.sol_vault.key();
        state.auth_bump = *bumps.get("auth").unwrap();
        state.vault_bump = *bumps.get("sol_vault").unwrap();
        state.state_bump = *bumps.get("state").unwrap();

        Ok(())
    }
}
