use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::state::SwapNftSol;
use crate::errors::SwapError;

#[derive(Accounts)]
pub struct Swap<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(mut)]
    pub swap_state: Account<'info, SwapNftSol>,
    #[account(mut)]
    pub nft_account: Account<'info, TokenAccount>,
    #[account(mut)]
    pub user_nft_account: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,


}

impl<'info> Swap<'info> {
    pub fn swap_sol_for_nft(ctx: Context<Swap>, amount_in: u64) -> Result<()> {
        let swap_state = &mut ctx.accounts.swap_state;

        if amount_in < swap_state.fee {
            return Err(SwapError::InsufficientAmount.into());
        }

        **ctx.accounts.user.to_account_info().try_borrow_mut_lamports()? -= amount_in;
        **ctx.accounts.swap_state.to_account_info().try_borrow_mut_lamports()? += amount_in;

        // Transfer the NFT from the swap_state to the user
        let cpi_accounts = Transfer {
            from: ctx.accounts.nft_account.to_account_info(),
            to: ctx.accounts.user_nft_account.to_account_info(),
            authority: ctx.accounts.swap_state.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::transfer(cpi_ctx, 1)?; // Transfer 1 NFT token

        Ok(())
    }
}
