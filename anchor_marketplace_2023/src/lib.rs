use anchor_lang::prelude::*;
use anchor_spl::{
    metadata::MetadataAccount,
    token::{Mint, Token},
};

declare_id!("VG58thf3atpUtDia6NUiJJeJLuHMy2z5CRiTBPXLzwT");

#[program]
pub mod anchor_marketplace_2023 {

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, fee: u16, name: String) -> Result<()> {
        require!(name.len() < 33 && name.len() > 3);
        ctx.accounts.marketplace.admin = ctx.accounts.admin.key();
        ctx.accounts.marketplace.fee = fee;
        ctx.accounts.name = name;
        Ok(())
    }

    pub fn add_collection(ctx: Context<AddCollection>) -> Result<()> {
        ctx.accounts.allowed_collection.collection = ctx.accounts.collection.key();
        Ok(())
    }

    pub fn list(ctx: Context<List>) -> Result<()> {
        unimplemented!()
    }

    // pub fn delist(ctx: Context<Initialize>) -> Result<()> {
    //     unimplemented!()
    // }

    // pub fn purchase(ctx: Context<Purchase>) -> Result<()> {
    //     unimplemented!()
    // }

    // pub fn offer(ctx: Context<Offer>) -> Result<()> {
    //     unimplemented!()
    // }
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        init,
        payer = admin,
        seeds = [b"marketplace", name.as_str().as_bytes()],
        space = Marketplace::LEN,
        bump,
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        init,
        payer = admin,
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = rewards
    )]
    rewards: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name: String)]
pub struct AddCollection<'info> {
    #[account(mut)]
    admin: Signer<'info>,
    #[account(
        has_one = admin,
        seeds = [b"marketplace", marketplace.name.as_str().as_bytes()],
        bump,
    )]
    marketplace: Account<'info, Marketplace>,
    collection_mint: Account<'info, >
    collection: Account<'info, MetadataAccount>,
    #[account(
        init,
        payer = admin,
        seeds = [b"collection", marketplace.key().as_ref(), collection.key().as_ref()],
        space = 8  + 32,
        bump
    )]
    allowed_collection: Account<'info, AllowedCollection>,
    system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct List<'info> {
    #[account(mut)]
    maker: Signer<'info>,
    #[account(
        seeds = [b"marketplace", name.as_str().as_bytes()],
        bump,
    )]
    marketplace: Account<'info, Marketplace>,
    #[account(
        seeds = [b"rewards", marketplace.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = rewards
    )]
    collection: Account<'info, Collection>,
    mint: Account<'info, Mint>,
    collection_mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

#[account]
pub struct Marketplace {
    admin: Pubkey,
    fee: u16,
    name: String,
}

impl Marketplace {
    const LEN: usize = 8 + 32 + 2 + 4 + 32;
}

#[account]
pub struct AllowedCollection {
    collection: Pubkey,
}

#[account]
pub struct Listing {
    owner: Pubkey,
    mint: Pubkey,
    price: u64,
    exp: u64,
}

#[error_code]
pub enum MarketplaceError {
    #[msg("Invalid Name")]
    InvalidName,
}
