use anchor_lang::prelude::*;

#[account]
pub struct NftMinting {
    pub maker: Pubkey,
    pub mint: Pubkey,
    pub price: u64,
    pub bump: u8,
}

impl Space for NftMinting {
    const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 1 + 1;
}