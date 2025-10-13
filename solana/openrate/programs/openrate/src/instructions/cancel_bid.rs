use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;

/// Cancel Bid instruction
///
/// Allows a lender to cancel an active bid and withdraw any
/// unfilled liquidity from the market vault.  
/// If the bid was partially filled, only the remaining amount
/// is returned. Once cancelled, the bid cannot be reused and
/// must be recreated via place_bid.
#[derive(Accounts)]
pub struct CancelBid<'info> {
    /// The lender cancelling their bid.
    #[account(mut, signer)]
    pub lender: Signer<'info>,

    /// The market associated with this bid.
    #[account(mut, has_one = vault, has_one = token_mint)]
    pub market: Account<'info, Market>,

    /// Vault metadata for this market.
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// The vault’s SPL token account.
    #[account(mut, constraint = vault_token_account.key() == vault.token_account)]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// The lender’s token account to receive the refund.
    #[account(mut, constraint = lender_token_account.mint == market.token_mint)]
    pub lender_token_account: Account<'info, TokenAccount>,

    /// The bid order being cancelled.
    #[account(mut, has_one = lender, has_one = market)]
    pub bid_order: Account<'info, BidOrder>,

    /// PDA that controls the vault; signs the token transfer.
    #[account(seeds = [b"vault_authority", market.key().as_ref()], bump = vault.bump)]
    pub vault_authority: SystemAccount<'info>,

    /// System and token programs.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// Cancel Bid instruction handler
///
/// Transfers any unfilled liquidity from the vault back to the
/// lender and marks the bid as inactive.  
/// The bid account remains on-chain for historical indexing but
/// is no longer considered active for matching.
pub fn cancel_bid(ctx: Context<CancelBid>) -> Result<()> {
    let vault = &ctx.accounts.vault;
    let bid_order = &mut ctx.accounts.bid_order;
    let market = &ctx.accounts.market;

    // Enforce: bid must be active and have remaining liquidity.
    require!(bid_order.is_active, ErrorCode::BidInactive);

    let unfilled_amount = bid_order.amount.saturating_sub(bid_order.filled_amount);
    require!(unfilled_amount > 0, ErrorCode::NoFundsToWithdraw);

    // Transfer remaining liquidity from vault to lender.
    let seeds = &[b"vault_authority", market.key().as_ref(), &[vault.bump]];
    let signer_seeds = &[&seeds[..]];

    let cpi_accounts = Transfer {
        from: ctx.accounts.vault_token_account.to_account_info(),
        to: ctx.accounts.lender_token_account.to_account_info(),
        authority: ctx.accounts.vault_authority.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer_seeds);
    token::transfer(cpi_ctx, unfilled_amount)?;

    // Deactivate bid and finalize state.
    bid_order.is_active = false;
    bid_order.filled_amount = bid_order.amount;

    Ok(())
}
