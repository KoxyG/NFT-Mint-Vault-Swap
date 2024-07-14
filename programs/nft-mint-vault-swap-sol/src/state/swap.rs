use anchor_lang::prelude::*;

pub struct Swap {
    pub authority: Pubkey,
    pub fee: u64,
}

impl Space for Swap {
    const INIT_SPACE: usize = 8 + 32 + 8;
}