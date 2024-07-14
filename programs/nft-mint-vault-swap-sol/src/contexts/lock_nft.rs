use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface, TransferChecked, transfer_checked};
use crate::state::NftMinting;

#[derive(Accounts)]
pub struct LockNft<'info> {
    #[account(mut)]
    user: Signer<'info>,
    #[account(mut)]
    user_ata: Account<'info, TokenAccount>,
    #[account(mut)]
    vault_ata: Account<'info, TokenAccount>,
    #[account(mut, address = nft_mint_address)]
    nft_mint: Account<'info, Mint>,
    #[account(seeds = [b"vault_authority"], bump)]
    vault_authority: AccountInfo<'info>,
    token_program: Program<'info, Token>,
}

impl<'info> LockNft<'info> {
    pub fn lock_nft(ctx: Context<LockNFT>, nft_mint_address: Pubkey, rental_fee: u64) -> Result<()> {
        let cpi_accounts = TransferChecked {
            from: ctx.accounts.user_ata.to_account_info(),
            to: ctx.accounts.vault_ata.to_account_info(),
            authority: ctx.accounts.user.to_account_info(),
            mint: ctx.accounts.nft_mint.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
        token::transfer_checked(cpi_ctx, 1, ctx.accounts.nft_mint.decimals)?;

        // Logic to handle rental fee
        // Implementation depends on your fee mechanism

        Ok(())
    }
}