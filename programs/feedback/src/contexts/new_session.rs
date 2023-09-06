use crate::{errors::ErrorCode, state::Session};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(
    name: String,
    website: String,
    description: String
)]
pub struct NewSession<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"session", owner.key().as_ref()],
        bump,
        space = Session::LEN
    )]
    pub session: Account<'info, Session>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewSession<'info> {
    pub fn new_session(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        name: String,
        website: String,
        description: String,
    ) -> Result<()> {
        require!(name.chars().count() <= 50, ErrorCode::NameTooLong);
        require!(website.chars().count() <= 50, ErrorCode::NameTooLong);
        require!(
            description.chars().count() <= 280,
            ErrorCode::ContentTooLong
        );
        let session = &mut self.session;
        session.bump = *bumps.get("session").unwrap();
        session.owner = self.owner.key();
        session.name = name;
        session.website = website;
        session.description = description;
        Ok(())
    }
}
