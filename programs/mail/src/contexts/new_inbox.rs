use crate::{errors::MailError, state::Inbox};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct NewInbox<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"inbox", owner.key().as_ref()],
        bump,
        space = Inbox::LEN
    )]
    pub inbox: Account<'info, Inbox>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewInbox<'info> {
    pub fn new_inbox(&mut self, bumps: &BTreeMap<String, u8>) -> Result<()> {
        // pub owner: Pubkey,
        // pub messages: u64,
        // pub created_at: i64,
        // pub bump: u8,

        let inbox = &mut self.inbox;
        inbox.owner = self.owner.key();
        inbox.messages = 0;
        inbox.created_at = Clock::get()?.unix_timestamp;
        inbox.bump = *bumps.get("inbox").unwrap();
        Ok(())
    }
}
