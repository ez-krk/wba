use crate::{
    errors::ErrorCode,
    state::Entry,
};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(input: String, seed: u64)]
pub struct NewEntry<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        seeds = [b"entry", owner.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = Entry::LEN + input.len(),
        bump,
    )]
    pub entry: Account<'info, Entry>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewEntry<'info> {
    pub fn new_entry(&mut self, bumps: &BTreeMap<String, u8>, input: String, seed: u64) -> Result<()> {
        if input.len() <= 0 {
            return err!(ErrorCode::EntryEmpty);
        }
        let entry = &mut self.entry;
        entry.owner = self.owner.key();
        entry.entry = input;
        entry.created_at = Clock::get().unwrap().unix_timestamp;
        entry.seed = seed;
        entry.bump = *bumps.get("entry").unwrap();
        Ok(())
    }
}
