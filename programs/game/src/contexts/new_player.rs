use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Game, Player},
};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
#[instruction(x: u8, y: u8)]
pub struct NewPlayer<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = Player::LEN,
        seeds = [b"player".as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub player: Account<'info, Player>,
    #[account(
        mut,
        seeds = [b"game"],
        bump,
    )]
    pub game: Account<'info, Game>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewPlayer<'info> {
    pub fn new_player(&mut self, bumps: &BTreeMap<String, u8>, x: u8, y: u8) -> Result<()> {
        if (x < 0 || y < 0) || (x > GRID_SIZE || y > GRID_SIZE) {
            return err!(ErrorCode::OutOfBounds);
        }
        let game = &mut self.game;
        game.alive += 1;
        let player = &mut self.player;
        player.player = self.owner.key();
        player.x = x;
        player.y = y;
        player.health = 100;
        player.kills = 0;
        player.shield = Clock::get()?.unix_timestamp + ONE_DAY_SECONDS;
        player.bump = *bumps.get("player").unwrap();
        Ok(())
    }
}
