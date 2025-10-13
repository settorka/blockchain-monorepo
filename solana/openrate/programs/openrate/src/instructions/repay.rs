use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;
use crate::errors::ErrorCode;

/// Repay instruction
///
/// Allows a borrower to repay a previously borrowed amount.
/// Transfers tokens from the borrower’s token account back into
/// the vault, marks the associated BorrowRecord as repaid, and
/// updates the corresponding BidOrder to reflect recovered liquidity.
///
/// For the MVP version, this instruction assumes full repayment
/// of the outstanding principal (no partial payments or interest accrual).
#[derive(Accounts)]
pub struct Repay<'info> {
    /// Borrower repaying the loan.
    #[account(mut, signer)]
    pub borrower: Signer<'info>,

    /// The market the borrow occurred in.
    #[account(mut, has_one = vault)]
    pub market: Account<'info, Market>,

    /// The vault metadata for this market.
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// The vault’s SPL token account holding all liquidity.
    #[account(mut, constraint = vault_token_account.key() == vault.token_account)]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// The borrower’s SPL token account to pull funds from.
    #[account(mut, constraint = borrower_token_account.mint == market.token_mint)]
    pub borrower_token_account: Account<'info, TokenAccount>,

    /// The borrow record being repaid.
    #[account(mut, has_one = borrower, has_one = market)]
    pub borrow_record: Account<'info, BorrowRecord>,

    /// The bid order originally funding this loan.
    #[account(mut, has_one = market)]
    pub bid_order: Account<'info, BidOrder>,

    /// Vault authority PDA (program signer for the vault).
    #[account(seeds = [b"vault_authority", market.key().as_ref()], bump = vault.bump)]
    pub vault_authority: SystemAccount<'info>,

    /// System and token programs.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// Repay instruction handler
///
/// Transfers the borrower’s repayment amount into the vault’s
/// SPL token account, marks the BorrowRecord as closed, and
/// reactivates the lender’s bid for reuse if applicable.
pub fn repay(ctx: Context<Repay>) -> Result<()> {
    let borrower = &ctx.accounts.borrower;

    let bid_order = &mut ctx.accounts.bid_order;
    let borrow_record = &mut ctx.accounts.borrow_record;

    // Enforce: loan must not already be repaid.
    require!(!borrow_record.repaid, ErrorCode::AlreadyRepaid);

    // Transfer the principal back to the vault.
    let amount = borrow_record.principal;
    let cpi_accounts = Transfer {
        from: ctx.accounts.borrower_token_account.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: borrower.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Mark loan as repaid.
    borrow_record.repaid = true;

    // Restore liquidity to the bid order.
    bid_order.filled_amount = bid_order.filled_amount.saturating_sub(amount);
    if bid_order.filled_amount < bid_order.amount {
        bid_order.is_active = true;
    }

    Ok(())
}
