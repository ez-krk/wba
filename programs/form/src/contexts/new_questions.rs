use crate::{constants::*, errors::ErrorCode, state::Questions};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(input: Vec<String>, seeds: u64)]
pub struct NewQuestions<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"questions", owner.key().as_ref(), seeds.to_le_bytes().as_ref()],
        bump,
        space = Questions::LEN + ((STRING_LENGTH_PREFIX + MAX_QUESTION_LENGTH) * input.len())
    )]
    pub questions: Account<'info, Questions>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewQuestions<'info> {
    pub fn new_questions(
        &mut self,
        bumps: &BTreeMap<String, u8>,
        input: Vec<String>,
    ) -> Result<()> {
        let questions = &mut self.questions;
        questions.bump = *bumps.get("questions").unwrap();
        questions.owner = self.owner.key();
        for x in input.iter() {
            if x.len() > MAX_QUESTION_LENGTH {
                return err!(ErrorCode::QuestionTooLong);
            } else if x.len() == 0 {
                return err!(ErrorCode::QuestionEmpty);
            } else {
                questions.questions.push(x.clone());
            }
        }
        questions.participants = 0;
        questions.created_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}
