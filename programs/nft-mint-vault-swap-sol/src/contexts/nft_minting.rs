use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, TokenAccount, Mint, Token, TransferChecked},
    associated_token::AssociatedToken,
    metadata::{Metadata, MetadataAccount},
};
use spl_token_metadata::instruction as token_metadata_instruction;
use solana_program::program::invoke;
use crate::state::NftMinting; 




 
#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(init, payer = user, mint::decimals = 0, mint::authority = mint_authority)]
    mint: Account<'info, Mint>,
    #[account(init, payer = user, associated_token::mint = mint, associated_token::authority = user)]
    token_account: Account<'info, TokenAccount>,
    #[account(mut)]
    user: Signer<'info>,
    #[account(seeds = [b"metadata"], bump)]
    metadata: Account<'info, MetadataAccount>,
    #[account(seeds = [b"authority"], bump)]
    mint_authority: AccountInfo<'info>,
    #[account(seeds = [b"metadata_program"], bump)]
    metadata_program: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

impl<'info> MintNFT<'info> {
    pub fn mint_nft(ctx: Context<MintNFT>, metadata_uri: String) -> Result<()> {
        let cpi_accounts = MintTo {
            mint: ctx.accounts.mint.to_account_info(),
            to: ctx.accounts.token_account.to_account_info(),
            authority: ctx.accounts.mint_authority.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::mint_to(cpi_ctx, 1)?;

        // Set metadata URI
        // This is a placeholder; you will need to use the appropriate program to set the metadata
        token_metadata_instruction(ctx.accounts.metadata_program.to_account_info(), ctx.accounts.metadata.to_account_info(), metadata_uri)?;

        Ok(())
    }
}