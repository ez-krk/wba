use crate::{
    errors::ErrorCode,
    state::{Feedback, Session, User},
};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(
    title: String,
    content: String,
    seed : u64
)]
pub struct NewFeedback<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user", owner.key().as_ref()],
        bump = user.bump,
    )]
    pub user: Account<'info, User>,
    #[account(
        seeds = [b"session", session.owner.as_ref()],
        bump = session.bump,
    )]
    pub session: Account<'info, Session>,
    #[account(
        init,
        payer = owner,
        seeds = [b"feedback", owner.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump,
        space = Feedback::LEN
    )]
    pub feedback: Account<'info, Feedback>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewFeedback<'info> {
    pub fn new_feedback(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        title: String,
        content: String,
        seed: u64,
    ) -> Result<()> {
        require!(title.chars().count() <= 50, ErrorCode::NameTooLong);
        require!(content.chars().count() <= 280, ErrorCode::NameTooLong);
        let feedback = &mut self.feedback;
        feedback.bump = *bumps.get("feedback").unwrap();
        feedback.user = self.user.key();
        feedback.session = self.session.key();
        feedback.title = title;
        feedback.content = content;
        feedback.helpful = false;
        feedback.seed = seed;
        feedback.created_at = Clock::get().unwrap().unix_timestamp;
        let user = &mut self.user;
        user.feedbacks += 1;
        user.reputation += 1;
        Ok(())
    }
}
