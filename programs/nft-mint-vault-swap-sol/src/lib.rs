use anchor_lang::prelude::*;

declare_id!("3nW5XsWpnJ7dYKmVhQQ9LiSSvw6CFQUe5kKgiZPxApMp");

pub mod state;
pub mod contexts;
pub mod errors;

pub use contexts::*;
pub use errors::*;

#[program]
pub mod nft_mint_vault_swap_sol {
    use super::*;

    pub fn nft_mint(ctx: Context<MintNFT>, metadata_uri: String) -> Result<()> {
        MintNFT::mint_nft(ctx, metadata_uri)
    }

    pub fn lock_nft(ctx: Context<LockNft>, nft_mint_address: Pubkey, rental_fee: u64) -> Result<()> {
        LockNft::lock_nft(ctx, nft_mint_address, rental_fee)
    }

    pub fn unlock_nft(ctx: Context<UnlockNFT>, nft_mint_address: Pubkey) -> Result<()> {
        UnlockNFT::unlock_nft(ctx, nft_mint_address)
    }

    pub fn swap(ctx: Context<Swap>, fee: u64) -> Result<()> {
        Swap::swap_sol_for_nft(ctx, fee)
    }
}
