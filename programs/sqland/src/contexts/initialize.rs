use crate::constants::*;
use crate::state::Game;
use anchor_lang::prelude::*;

use anchor_spl::{
    metadata::{create_metadata_accounts_v3, CreateMetadataAccountsV3, Metadata, MetadataAccount},
    token::{Mint, Token},
};
use mpl_token_metadata::state::DataV2;
use std::collections::BTreeMap;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut,address = ADMIN_PUBKEY)]
    pub owner: Signer<'info>,
    #[account(
        init,
        payer = owner,
        space = Game::LEN,
        seeds = [b"game"],
        bump,
    )]
    pub game: Account<'info, Game>,
    // The PDA is both the address of the mint account and the mint authority
    #[account(
        init,
        seeds = [b"mint"],
        bump,
        payer = owner,
        mint::decimals = 6,
        mint::authority = mint,

    )]
    pub mint: Account<'info, Mint>,

    ///CHECK: Using "address" constraint to validate metadata account address
    #[account(
        mut,
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            mint.key().as_ref()
        ],
        bump,
        seeds::program = metadata_program.key()
    )]
    /// CHECK: metaplex will check this
    pub metadata_account: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
    pub token_metadata_program: Program<'info, Metadata>,
    pub metadata_program: Program<'info, Metadata>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

impl<'info> Initialize<'info> {
    pub fn init(&mut self, bumps: &BTreeMap<String, u8>) -> Result<()> {
        let game = &mut self.game;
        game.alive = 0;
        game.dead = 0;
        game.mint = self.mint.key();
        game.token_minted = 0;
        game.token_burnt = 0;
        game.auth_bump = *bumps.get("auth").unwrap();
        game.sol_vault_bump = *bumps.get("sol_vault").unwrap();
        game.spl_vault_bump = *bumps.get("spl_vault").unwrap();
        Ok(())
    }

    // pub fn create_mint(
    //     &self,
    //     bumps: &BTreeMap<String, u8>,
    //     uri: String,
    //     name: String,
    //     symbol: String,
    // ) -> Result<()> {
    //     // PDA seeds and bump to "sign" for CPI
    //     let seeds: &[u8; 4] = b"mint";
    //     let bump: u8 = *bumps.get("mint").unwrap();
    //     let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    //     // On-chain token metadata for the mint
    //     let data_v2: DataV2 = DataV2 {
    //         name,
    //         symbol,
    //         uri,
    //         seller_fee_basis_points: 0,
    //         creators: None,
    //         collection: None,
    //         uses: None,
    //     };

    //     // CPI Context
    //     let cpi_ctx = CpiContext::new_with_signer(
    //         self.token_metadata_program.to_account_info(),
    //         CreateMetadataAccountsV3 {
    //             metadata: self.metadata_account.to_account_info(), // the metadata account being created
    //             mint: self.mint.to_account_info(), // the mint account of the metadata account
    //             mint_authority: self.mint.to_account_info(), // the mint authority of the mint account
    //             update_authority: self.mint.to_account_info(), // the update authority of the metadata account
    //             payer: self.owner.to_account_info(), // the payer for creating the metadata account
    //             system_program: self.system_program.to_account_info(), // the system program account
    //             rent: self.rent.to_account_info(),   // the rent sysvar account
    //         },
    //         signer,
    //     );

    //     create_metadata_accounts_v3(
    //         cpi_ctx, // cpi context
    //         data_v2, // token metadata
    //         true,    // is_mutable
    //         true,    // update_authority_is_signer
    //         None,    // collection details
    //     )
    // }
}
