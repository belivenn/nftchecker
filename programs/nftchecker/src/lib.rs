use anchor_lang::prelude::*;
use anchor_spl::{
    token_interface::{TokenAccount, Mint, TokenInterface}, 
    metadata::{Metadata, MetadataAccount, MasterEditionAccount}, 
    associated_token::AssociatedToken
};


declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nftchecker {
    use super::*;

    pub fn nft_checker(ctx: Context<NftChecker>) -> Result<()> {
        ctx.accounts.nft_checker(&ctx.bumps)?;
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
        init,
        payer = owner,
        space = OwnerData::INIT_SPACE,
        seeds = [owner.key().as_ref(), nft.key().as_ref()],
        bump
    )]
    data: Account<'info, OwnerData>,
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
    pub fn nft_checker(&mut self, bumps: &NftCheckerBumps) -> Result<()> {
        self.data.set_inner(OwnerData {
            owner: self.owner.key(),
            mint: self.nft.key(),
            bump: bumps.data,
        });
        Ok(())
    }
}

#[account]
pub struct OwnerData {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub bump: u8,
}

impl Space for OwnerData {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1;
}
