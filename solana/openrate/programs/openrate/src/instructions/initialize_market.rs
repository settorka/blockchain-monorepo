use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::*;

/// Initialize Market instruction
///
/// Sets up a new lending market for a specific SPL token.
/// Creates and initializes:
/// - the `Market` account (stores market config)
/// - the `Vault` account (holds all liquidity for this market)
/// The initializer becomes the market authority and funds the account creations.
#[derive(Accounts)]
pub struct InitializeMarket<'info> {
    /// Payer and market authority; funds account creation.
    #[account(mut, signer)]
    pub authority: Signer<'info>,

    /// Market configuration account (PDA).
    #[account(
        init,
        payer = authority,
        space = Market::LEN,
        seeds = [b"market", token_mint.key().as_ref()],
        bump
    )]
    pub market: Account<'info, Market>,

    /// Vault metadata account (PDA).
    #[account(
        init,
        payer = authority,
        space = Vault::LEN,
        seeds = [b"vault", token_mint.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, Vault>,

    /// SPL token mint for this market.
    pub token_mint: Account<'info, Mint>,

    /// Actual SPL token account that will serve as the vault’s balance holder.
    #[account(
        init,
        payer = authority,
        token::mint = token_mint,
        token::authority = vault_authority,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// PDA authority for vault control.
    #[account(seeds = [b"vault_authority", market.key().as_ref()], bump)]
    pub vault_authority: SystemAccount<'info>,

    /// Standard Solana programs.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// Initialize Market instruction handler
pub fn initialize_market(ctx: Context<InitializeMarket>) -> Result<()> {
    // Get bumps (no deref)
    let market_bump = ctx.bumps.market;
    let vault_bump = ctx.bumps.vault;

    // Set market config
    ctx.accounts.market.authority = ctx.accounts.authority.key();
    ctx.accounts.market.token_mint = ctx.accounts.token_mint.key();
    ctx.accounts.market.vault = ctx.accounts.vault.key();
    ctx.accounts.market.bump = market_bump;
    ctx.accounts.market._reserved = [0; 7];

    // Set vault config
    ctx.accounts.vault.authority = ctx.accounts.vault_authority.key();
    ctx.accounts.vault.token_account = ctx.accounts.vault_token_account.key();
    ctx.accounts.vault.token_mint = ctx.accounts.token_mint.key();
    ctx.accounts.vault.bump = vault_bump;
    ctx.accounts.vault._reserved = [0; 7];

    Ok(())
}
