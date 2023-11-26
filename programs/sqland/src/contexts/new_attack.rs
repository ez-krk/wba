use crate::{
    errors::ErrorCode,
    state::{Attack, Player},
};
use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct NewAttack<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        has_one = owner,
        seeds = [b"player".as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub player: Account<'info, Player>,
    #[account(
        mut,
        seeds = [b"player".as_ref(), target.owner.as_ref()],
        bump,
    )]
    pub target: Account<'info, Player>,
    #[account(
        mut,
        seeds = [b"attack".as_ref(), owner.key().as_ref(), target.owner.as_ref()],
        bump,
    )]
    pub attack: Account<'info, Attack>,
    pub system_program: Program<'info, System>,
}

impl<'info> NewAttack<'info> {
    pub fn new_attack(&mut self, bumps: &BTreeMap<String, u8>) -> Result<()> {
        let player = &mut self.player;
        let target = &mut self.target;
        let attack = &mut self.attack;
        match player.idle {
            true => match target.idle {
                // here both players are idle
                true => {
                    // ensure target ain't shielded
                    if check_shield(target.shield) == false {
                        attack.bump = *bumps.get("attack").unwrap();
                        attack.started_at = Clock::get()?.unix_timestamp;
                        attack.player = player.owner.key();
                        attack.target = target.owner.key();
                        attack.resolved = false;
                    } else {
                        return err!(ErrorCode::TargetShielded);
                    }
                }
                false => return err!(ErrorCode::TargetNotIdle),
            },
            false => return err!(ErrorCode::PlayerNotIdle),
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
