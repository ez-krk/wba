use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod state;

use contexts::*;

declare_id!("DEVK1cQcDUsCwnE1rZvKvEeg4GmPo4SSfmV3cb1TEw2u");

#[program]
pub mod form {
    use super::*;

    pub fn new_form(ctx: Context<NewQuestions>, input: Vec<String>) -> Result<()> {
        ctx.accounts.new_questions(&ctx.bumps, input)
    }

    pub fn new_answer(ctx: Context<NewAnswers>, input: Vec<String>) -> Result<()> {
        ctx.accounts.new_answers(&ctx.bumps, input)
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided name should be 50 characters long maximum")]
    NameTooLong,
    #[msg("The provided content should be 280 characters long maximum")]
    ContentTooLong,
}
