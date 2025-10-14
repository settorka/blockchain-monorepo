use anchor_lang::prelude::*;

/// BidOrder account
///
/// Represents a lender’s offer to lend tokens at a specific interest rate.
/// Orders can be partially or fully filled when borrowers request loans
/// that meet or exceed the bid’s rate.
#[account]
pub struct BidOrder {
    pub lender: Pubkey,
    pub market: Pubkey,
    pub amount: u64,
    pub rate_bps: u16,
    pub filled_amount: u64,
    pub bump: u8,
    pub is_active: bool,
    pub created_at: i64,
    pub _reserved: [u8; 6],
}

/// BidOrder implementation
///
/// The LEN constant ensures deterministic account sizing for Solana's
/// rent-exemption calculation.  
/// - 8 bytes → Anchor discriminator identifying account type  
/// - 32 + 32 → lender and market public keys  
/// - 8 → u64 token amount  
/// - 2 → u16 interest rate in basis points  
/// - 8 → u64 filled amount  
/// - 1 → bump seed for PDA derivation  
/// - 1 → boolean is_active flag  
/// - 8 → i64 Unix timestamp of creation  
/// - 6 → padding for alignment and forward compatibility
impl BidOrder {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 2 + 8 + 1 + 1 + 8 + 6;
}
