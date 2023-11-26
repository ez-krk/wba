use crate::{
    errors::MailError,
    state::{Inbox, Mail},
};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(
    message: Vec<u8>,
    seed : u128
)]
pub struct NewMail<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    #[account(
        mut,
        seeds = [b"inbox", inbox.owner.as_ref()],
        bump = inbox.bump,
    )]
    pub inbox: Account<'info, Inbox>,
    #[account(
        init,
        payer = sender,
        seeds = [b"mail", sender.key().as_ref(), inbox.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = Mail::LEN + message.len()
    )]
    pub mail: Account<'info, Mail>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewMail<'info> {
    pub fn new_mail(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        message: Vec<u8>,
        seed: u128,
    ) -> Result<()> {
        // require!(message.chars().count() <= 4096, MailError::MessageTooLong);
        // require!(content.chars().count() <= 280, ErrorCode::NameTooLong);

        let mail = &mut self.mail;

        // pub sender: Pubkey,
        // pub inbox: Pubkey,
        // pub message: Vec<u8>,
        // pub created_at: i64,
        // pub seed: u128,
        // pub bump: u8,

        mail.sender = self.sender.key();
        mail.inbox = self.inbox.key();
        mail.message = message;
        mail.created_at = Clock::get().unwrap().unix_timestamp;
        mail.seed = seed;
        mail.bump = *bumps.get("mail").unwrap();
        Ok(())
    }
}
