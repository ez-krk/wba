use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
mod state;

use contexts::*;

declare_id!("DEVK1cQcDUsCwnE1rZvKvEeg4GmPo4SSfmV3cb1TEw2u");

#[program]
pub mod input {
    use super::*;

    pub fn new_questions(ctx: Context<NewQuestions>, input: Vec<String>, seed: u64) -> Result<()> {
        ctx.accounts.new_questions(&ctx.bumps, input, seed)
    }

    pub fn new_answers(ctx: Context<NewAnswers>, input: Vec<String>) -> Result<()> {
        ctx.accounts.new_answers(&ctx.bumps, input)
    }
}
