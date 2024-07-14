use anchor_lang::prelude::*;

#[account]
#[derive(Default,)]
pub struct SwapNftSol {
    pub authority: Pubkey,
    pub fee: u64,

}

impl Space for SwapNftSol {
    const INIT_SPACE: usize = 8 + 32 + 8;
}