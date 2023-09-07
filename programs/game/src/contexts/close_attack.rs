use crate::{
    constants::*,
    errors::ErrorCode,
    state::{Attack, Game, Player},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{mint_to, Mint, MintTo, Token, TokenAccount},
};
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct CloseAttack<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        mut,
        seeds = [b"game"],
        bump,
    )]
    pub game: Account<'info, Game>,
    #[account(
        mut,
        has_one = player,
        seeds = [b"attack".as_ref(), signer.key().as_ref(), attack.target.as_ref()],
        bump,
    )]
    pub attack: Account<'info, Attack>,
    #[account(
        mut,
        has_one = player,
        seeds = [b"player".as_ref(), attack.player.as_ref()],
        bump,
    )]
    pub player: Account<'info, Player>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    pub player_ata: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"player".as_ref(), attack.target.as_ref()],
        bump,
    )]
    pub target: Account<'info, Player>,
    #[account(
        mut,
        seeds = [b"mint"],
        bump,
    )]
    pub mint: Box<Account<'info, Mint>>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> CloseAttack<'info> {
    pub fn close_attack(&mut self, bumps: &BTreeMap<String, u8>) -> Result<()> {
        let player = &mut self.player;
        let target = &mut self.target;
        let attack = &mut self.attack;
        let game = &mut self.game;
        match Clock::get()?.unix_timestamp >= (attack.started_at + ONE_DAY_SECONDS) {
            true => {
                // ensure target ain't shielded
                if check_shield(target.shield) == false {
                    attack.player = player.player.key();
                    attack.target = target.player.key();
                    attack.started_at = Clock::get()?.unix_timestamp;
                    // check if we kill, if so mint target.wins as tokens to attacker
                    if target.health.checked_sub(10).unwrap() <= 0 {
                        // cpi call
                        let seeds: &[u8; 6] = b"reward";
                        let bump: u8 = *bumps.get("mint").unwrap();
                        let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];
                        let cpi_ctx = CpiContext::new_with_signer(
                            self.token_program.to_account_info(),
                            MintTo {
                                mint: self.mint.to_account_info(),
                                to: self.player_ata.to_account_info(),
                                authority: self.mint.to_account_info(),
                            },
                            signer,
                        );
                        let amount = (1u64)
                            .checked_mul(target.kills * 10u64.pow(self.mint.decimals as u32))
                            .unwrap();
                        mint_to(cpi_ctx, amount);
                        player.kills += 1;
                        game.alive -= 1;
                        game.dead += 1;
                    // we don't kill, we sub 10 hp
                    } else if target.health.checked_sub(10).unwrap() > 0 {
                        target.health = target.health.checked_sub(10).unwrap();
                    } else {
                        return err!(ErrorCode::TargetDead);
                    }
                } else {
                    return err!(ErrorCode::TargetShielded);
                }
            }
            false => return err!(ErrorCode::TooEarly),
        }
        Ok(())
    }
}

pub fn check_shield(tz: i64) -> bool {
    let clock = Clock::get().unwrap();
    let current_timestamp = clock.unix_timestamp;
    let mut shield: bool = true;
    if current_timestamp > tz {
        msg!("Shield over !");
        shield = false;
    } else if current_timestamp < tz {
        msg!("Shield active !");
        shield = true;
    }
    shield
}
