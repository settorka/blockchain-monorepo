use anchor_lang::prelude::*;

/// BorrowRecord account
///
/// Tracks a borrower's active or completed loan position within
/// a specific market. Created when a borrower receives funds and
/// closed once repayment is confirmed.
#[account]
pub struct BorrowRecord {
    pub borrower: Pubkey,
    pub market: Pubkey,
    pub principal: u64,
    pub rate_bps: u16,
    pub start_time: i64,
    pub repaid: bool,
    pub bump: u8,
    pub _reserved: [u8; 6],
}

/// BorrowRecord implementation
///
/// `LEN` defines the precise byte layout of this account for rent
/// exemption and deterministic PDA allocations.  
/// - `8` bytes → Anchor discriminator  
/// - `32 + 32` → borrower and market public keys  
/// - `8` → u64 principal amount  
/// - `2` → u16 rate in basis points  
/// - `8` → i64 start timestamp  
/// - `1` → boolean repayment status  
/// - `1` → PDA bump seed  
/// - `6` → padding to maintain 8-byte alignment
impl BorrowRecord {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 2 + 8 + 1 + 1 + 6;
}
