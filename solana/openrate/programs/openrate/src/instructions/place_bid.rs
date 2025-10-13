use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};
use crate::state::*;

/// Place Bid instruction
///
/// Allows a lender to deposit tokens into the market vault and
/// register a new BidOrder offer specifying their desired rate.
/// The deposited amount is locked in the vault until the bid is
/// filled or cancelled.
#[derive(Accounts)]
pub struct PlaceBid<'info> {
    /// The lender placing the bid; provides and signs the token transfer.
    #[account(mut, signer)]
    pub lender: Signer<'info>,

    /// The market this bid belongs to.
    #[account(mut, has_one = token_mint)]
    pub market: Account<'info, Market>,

    /// Vault metadata for this market.
    #[account(mut)]
    pub vault: Account<'info, Vault>,

    /// The lender’s SPL token account holding the funds to lend.
    #[account(mut, constraint = lender_token_account.mint == market.token_mint)]
    pub lender_token_account: Account<'info, TokenAccount>,

    /// The SPL token account that serves as the vault’s balance holder.
    #[account(mut, constraint = vault_token_account.key() == vault.token_account)]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// The BidOrder PDA to be created for this offer.
    #[account(
        init,
        payer = lender,
        space = BidOrder::LEN,
        seeds = [b"bid_order", lender.key().as_ref(), market.key().as_ref()],
        bump
    )]
    pub bid_order: Account<'info, BidOrder>,

    /// The program-derived authority that controls the vault.
    #[account(seeds = [b"vault_authority", market.key().as_ref()], bump = vault.bump)]
    pub vault_authority: SystemAccount<'info>,

    /// System and token programs.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// Place Bid instruction handler
///
/// Transfers the lender’s specified amount of tokens into the vault,
/// creates a new BidOrder account, and stores metadata about the
/// offer, including the rate (in basis points) and timestamp.
pub fn place_bid(ctx: Context<PlaceBid>, amount: u64, rate_bps: u16) -> Result<()> {
    let lender = &ctx.accounts.lender;
    let vault = &ctx.accounts.vault;
    let market = &ctx.accounts.market;
    let bid_order = &mut ctx.accounts.bid_order;
    let clock = Clock::get()?;

    // Move lender’s tokens into the vault.
    let cpi_accounts = Transfer {
        from: ctx.accounts.lender_token_account.to_account_info(),
        to: ctx.accounts.vault_token_account.to_account_info(),
        authority: lender.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(ctx.accounts.token_program.to_account_info(), cpi_accounts);
    token::transfer(cpi_ctx, amount)?;

    // Populate bid order data.
    let bid_bump = *ctx.bumps.get("bid_order").unwrap();
    bid_order.lender = lender.key();
    bid_order.market = market.key();
    bid_order.amount = amount;
    bid_order.rate_bps = rate_bps;
    bid_order.filled_amount = 0;
    bid_order.bump = bid_bump;
    bid_order.is_active = true;
    bid_order.created_at = clock.unix_timestamp;
    bid_order._reserved = [0; 6];

    Ok(())
}
