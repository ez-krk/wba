use crate::state::Questions;
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(questions: Vec<String>, seeds: u64)]
pub struct NewQuestions<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"form", owner.key().as_ref(), seeds.to_le_bytes().as_ref()],
        bump,
        space = Questions::LEN
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
        questions.bump = *bumps.get("form").unwrap();
        questions.owner = self.owner.key();
        for x in input.iter() {
            questions.questions.push(x.clone())
        }
        questions.participants = 0;
        Ok(())
    }
}
