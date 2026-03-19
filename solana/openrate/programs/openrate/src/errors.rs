use anchor_lang::prelude::*;

/// Custom errors for the OpenRate program.
#[error_code]
pub enum ErrorCode {
    /// The referenced bid is no longer active or has been fully filled.
    BidInactive = 6000,
    /// The bid does not have enough remaining liquidity to satisfy this borrow.
    InsufficientBidLiquidity,
    /// The borrower attempted to withdraw funds when none are available.
    NoFundsToWithdraw,
    /// The provided token mint does not match the expected market mint.
    InvalidTokenMint,
    /// The vault authority or bump seeds are invalid.
    InvalidVaultAuthority,
    /// The account bump could not be derived or retrieved.
    MissingBump,
    /// The borrower does not own the specified account.
    UnauthorizedBorrower,
    /// This borrow has already been repaid.
    AlreadyRepaid,
    /// Overflow or underflow during arithmetic operation.
    MathError,

}
