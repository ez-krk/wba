use anchor_lang::prelude::*;

mod constants;
mod contexts;
mod errors;
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
