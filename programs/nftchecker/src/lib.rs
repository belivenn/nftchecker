use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{TokenAccount, Mint, TokenInterface}, 
    metadata::{Metadata, MetadataAccount, MasterEditionAccount}, 
    associated_token::AssociatedToken
};
use anchor_lang::prelude::error_code;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nftchecker {
    use super::*;

    pub fn nft_checker(ctx: Context<NftChecker>) -> Result<()> {
        ctx.accounts.nft_checker()?;
        msg!("This nft is from a collection");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct NftChecker<'info> {
    #[account(mut)]
    owner: Signer<'info>,
    nft: InterfaceAccount<'info, Mint>,
    collection_mint: InterfaceAccount<'info, Mint>,
    #[account(
        associated_token::mint = nft,
        associated_token::authority = owner
    )]
    owner_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            nft.key().as_ref()
        ],
        seeds::program = metadata_program.key(),
        bump,
        constraint = metadata.collection.as_ref().unwrap().key.as_ref() == collection_mint.key().as_ref(),
        constraint = metadata.collection.as_ref().unwrap().verified == true,
    )]
    metadata: Account<'info, MetadataAccount>,
    #[account(
        seeds = [
            b"metadata",
            metadata_program.key().as_ref(),
            nft.key().as_ref(),
            b"edition"
        ],
        seeds::program = metadata_program.key(),
        bump,
    )]
    master_edition: Account<'info, MasterEditionAccount>,
    metadata_program: Program<'info, Metadata>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
    token_program: Interface<'info, TokenInterface>,
}

impl<'info> NftChecker<'info> {
    pub fn nft_checker(&mut self) -> Result<()> {
        validate_nft!(
            self.metadata.collection, 
            self.collection_mint
            );
            Ok(())
    }
}

#[error_code]
pub enum Error {
    #[msg("Collection Not Set")]
    CollectionNotSet,
    #[msg("Invalid Collection")]
    InvalidCollection,

}

#[macro_export]
macro_rules! validate_nft {
    ($metadata:expr,$collection_mint:expr) => {
        require!(
            $metadata.is_some(),
            Error::CollectionNotSet
        );

        require_keys_eq!(
            $metadata.clone().unwrap().key,
            $collection_mint.key(),
            Error::InvalidCollection
        );

        require!(
            $metadata.clone().unwrap().verified,
            Error::InvalidCollection
        );
    };
}