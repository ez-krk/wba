use anchor_lang::prelude::*;

declare_id!("FEEDLQWp85GJBrum4ASAR6WEAW595oQfaST1N84HZFbS");

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_NAME_LENGTH: usize = 50 * 4; // 50 chars max.
const MAX_CONTENT_LENGTH: usize = 280 * 4; // 280 chars max.
const BUMP_LENGTH: usize = 1;
const BOOL_LENGTH: usize = 1;

#[program]
pub mod feedback {
    use super::*;

    pub fn new_session(
        ctx: Context<NewSession>,
        name: String,
        website: String,
        description: String,
    ) -> Result<()> {
        require!(name.chars().count() <= 50, ErrorCode::NameTooLong);
        require!(website.chars().count() <= 50, ErrorCode::NameTooLong);
        require!(
            description.chars().count() <= 280,
            ErrorCode::ContentTooLong
        );
        let session = &mut ctx.accounts.session;
        session.bump = *ctx.bumps.get("session").unwrap();
        session.owner = ctx.accounts.owner.key();
        session.name = name;
        session.website = website;
        session.description = description;
        Ok(())
    }

    pub fn new_feedback(
        ctx: Context<NewFeedback>,
        title: String,
        content: String,
        seed: u64,
    ) -> Result<()> {
        require!(title.chars().count() <= 50, ErrorCode::NameTooLong);
        require!(content.chars().count() <= 280, ErrorCode::NameTooLong);
        let feedback = &mut ctx.accounts.feedback;
        feedback.bump = *ctx.bumps.get("feedback").unwrap();
        feedback.user = ctx.accounts.user.key();
        feedback.session = ctx.accounts.session.key();
        feedback.title = title;
        feedback.content = content;
        feedback.helpful = false;
        feedback.seed = seed;
        feedback.created_at = Clock::get().unwrap().unix_timestamp;
        let user = &mut ctx.accounts.user;
        user.feedbacks += 1;
        user.reputation += 1;
        Ok(())
    }

    pub fn new_user(ctx: Context<NewUser>) -> Result<()> {
        let user = &mut ctx.accounts.user;
        user.bump = *ctx.bumps.get("user").unwrap();
        user.owner = ctx.accounts.owner.key();
        user.reputation = 0;
        user.feedbacks = 0;
        user.created_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(
    name: String,
    website: String,
    description: String
)]
pub struct NewSession<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"session", owner.key().as_ref()],
        bump,
        space = Session::LEN
    )]
    pub session: Account<'info, Session>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Session {
    pub owner: Pubkey,
    pub name: String,
    pub website: String,
    pub description: String,
    pub bump: u8,
}

impl Session {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner.
        + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH // name.
        + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH // website.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH // description.
        + BUMP_LENGTH; // bump.
}

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

#[account]
pub struct Feedback {
    pub user: Pubkey,
    pub session: Pubkey,
    pub title: String,
    pub content: String,
    pub helpful: bool,
    pub created_at: i64,
    pub seed: u64,
    pub bump: u8,
}

impl Feedback {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner.
        + PUBLIC_KEY_LENGTH // session.
        + STRING_LENGTH_PREFIX + MAX_NAME_LENGTH // title.
        + STRING_LENGTH_PREFIX + MAX_CONTENT_LENGTH // content.
        + BOOL_LENGTH // helpful.
        + TIMESTAMP_LENGTH // created_at.
        + 8 // seed.
        + BUMP_LENGTH; // bump.
}

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

#[account]
pub struct User {
    pub owner: Pubkey,
    pub reputation: u64,
    pub feedbacks: u64,
    pub created_at: i64,
    pub bump: u8,
}

impl User {
    const LEN: usize = DISCRIMINATOR_LENGTH
        + PUBLIC_KEY_LENGTH // owner.
        + 8 // reputation.
        + 8 // feedbacks.
        + TIMESTAMP_LENGTH // created_at.
        + BUMP_LENGTH; // bump.
}

#[error_code]
pub enum ErrorCode {
    #[msg("The provided name should be 50 characters long maximum")]
    NameTooLong,
    #[msg("The provided content should be 280 characters long maximum")]
    ContentTooLong,
}
