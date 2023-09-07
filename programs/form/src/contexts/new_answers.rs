use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Answers, Questions},
};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(input: Vec<String>)]
pub struct NewAnswers<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"questions", questions.owner.as_ref()],
        bump,
    )]
    pub questions: Account<'info, Questions>,
    #[account(
        init,
        payer = owner,
        seeds = [b"answers", questions.key().as_ref(), owner.key().as_ref()],
        bump,
        space = Answers::LEN + ((STRING_LENGTH_PREFIX + MAX_ANSWER_LENGTH) * input.len())
    )]
    pub answers: Account<'info, Answers>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewAnswers<'info> {
    pub fn new_answers(&mut self, bumps: &BTreeMap<String, u8>, input: Vec<String>) -> Result<()> {
        let answers = &mut self.answers;
        answers.bump = *bumps.get("answer").unwrap();
        answers.owner = self.owner.key();
        answers.created_at = Clock::get().unwrap().unix_timestamp;

        for x in input.iter() {
            if x.len() > MAX_ANSWER_LENGTH {
                return err!(ErrorCode::AnswerTooLong);
            } else if x.len() == 0 {
                return err!(ErrorCode::AnswerEmpty);
            } else {
                answers.answers.push(x.clone());
            }
        }
        let questions = &mut self.questions;
        questions.participants += 1;
        Ok(())
    }
}
