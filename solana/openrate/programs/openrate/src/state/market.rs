use anchor_lang::prelude::*;

/// Market account
///
/// Defines the configuration for a single lending market.
/// Each market is tied to one SPL token mint (e.g., USDC)
/// and maintains a vault for all lender deposits and borrower loans.
#[account]
pub struct Market {
    pub authority: Pubkey,
    pub token_mint: Pubkey,
    pub vault: Pubkey,
    pub bump: u8,
    pub _reserved: [u8; 7],
}

/// Market account implementation
///
/// number of bytes required to allocate this account on Solana. 
/// 32 bytes. 
/// The 1-byte bump seed is used for PDA derivation, and the
/// 7-byte reserved array keeps the account size aligned to 8-byte
/// boundaries, leaving room for future upgrades.
impl Market {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 1 + 7;
}
