use crate::state::User;
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct NewUser<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"user", owner.key().as_ref()],
        bump,
        space = User::LEN
    )]
    user: Account<'info, User>,
    system_program: Program<'info, System>,
}

impl<'info> NewUser<'info> {
    pub fn new_user(&mut self, bumps: &BTreeMap<String, u8>) -> Result<()> {
        let user = &mut self.user;
        user.bump = *bumps.get("user").unwrap();
        user.owner = self.owner.key();
        user.reputation = 0;
        user.feedbacks = 0;
        user.created_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}
