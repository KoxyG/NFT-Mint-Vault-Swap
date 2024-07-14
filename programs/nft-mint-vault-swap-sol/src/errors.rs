use anchor_lang::error_code;

#[error_code]
pub enum MarketplaceError {
    #[msg("Name must be between 1 and 32 characters")]
    NameTooLong,
}

#[error_code]
pub enum SwapError {
    #[msg("Insufficient amount provided.")]
    InsufficientAmount, 
}

#[error_code]
pub enum MintError {
    #[msg("NFT minting failed.")]
    NftMintingFailed,
    #[msg("Invalid metadata provided.")]
    InvalidMetadata,
    #[msg("Unauthorized minting action.")]
    Unauthorized,
}

#[error_code]
pub enum LockError {
    #[msg("NFT locking failed.")]
    NftLockingFailed,
    #[msg("Unauthorized locking action.")]
    Unauthorized,
    #[msg("Insufficient funds for locking NFT.")]
    InsufficientFunds,
}

#[error_code]
pub enum UnlockError {
    #[msg("NFT unlocking failed.")]
    NftUnlockingFailed,
    #[msg("Unauthorized unlocking action.")]
    Unauthorized,
}
