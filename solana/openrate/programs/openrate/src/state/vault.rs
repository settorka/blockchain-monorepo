use anchor_lang::prelude::*;

/// Vault account
///
/// Stores metadata for the token vault controlled by the program.
/// The vault is an SPL Token account owned by the program’s PDA
/// that holds all lender deposits and borrower repayments.
#[account]
pub struct Vault {
    pub authority: Pubkey,
    pub token_account: Pubkey,
    pub token_mint: Pubkey,
    pub bump: u8,
    pub _reserved: [u8; 7],
}

/// Vault implementation
///
/// Explains account sizing and alignment:  
/// - 8 bytes → Anchor discriminator  
/// - 32 + 32 + 32 → three public keys (authority, token account, mint)  
/// - 1 → PDA bump seed  
/// - 7 → padding for 8-byte alignment and upgrade flexibility  
/// This predictable size ensures rent-exemption calculations remain stable
/// across client versions.
impl Vault {
    pub const LEN: usize = 8 + 32 + 32 + 32 + 1 + 7;
}
