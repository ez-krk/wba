use crate::{
    constants::*,
    state::{Game, Player},
};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{burn, Burn, Mint, Token, TokenAccount},
};
#[derive(Accounts)]
pub struct Heal<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    #[account(
        mut,
        seeds = [b"game"],
        bump,
    )]
    pub game: Account<'info, Game>,
    #[account(
        mut,
        seeds = [b"player".as_ref(), owner.key().as_ref()],
        bump,
    )]
    pub player: Account<'info, Player>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = owner
    )]
    pub player_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"reward"],
        bump,
    )]
    pub mint: Account<'info, Mint>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Heal<'info> {
    pub fn heal(&mut self) -> Result<()> {
        self.player.health = MAX_HEALTH;

        let cpi_ctx = CpiContext::new(
            self.token_program.to_account_info(),
            Burn {
                mint: self.mint.to_account_info(),
                from: self.player_ata.to_account_info(),
                authority: self.owner.to_account_info(),
            },
        );

        // burn 1 token, accounting for mint decimals
        let amount = (1u64)
            .checked_mul(10u64.pow(self.mint.decimals as u32))
            .unwrap();

        burn(cpi_ctx, amount)
    }
}
