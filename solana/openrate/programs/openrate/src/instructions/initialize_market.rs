use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
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

    /// PDA authority for vault control (derived inside instruction).
    /// This will sign all token CPI transfers on behalf of the program.
    /// Derived via: `["vault_authority", market.key().as_ref()]`
    /// Created implicitly via PDA; not stored on chain.
    /// Used only to derive bump for Vault metadata.
    /// (No `init` since it's not an account itself)
    #[account(seeds = [b"vault_authority", market.key().as_ref()], bump)]
    pub vault_authority: SystemAccount<'info>,

    /// Standard Solana programs.
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

/// Initialize Market instruction handler
///
/// Allocates and seeds the `Market` and `Vault` accounts.
/// Derives their PDAs, stores configuration data, and links
/// the token mint, vault account, and authority for subsequent
/// lending operations.
pub fn initialize_market(ctx: Context<InitializeMarket>) -> Result<()> {
    let market = &mut ctx.accounts.market;
    let vault = &mut ctx.accounts.vault;

    // Derive PDAs
    let market_bump = *ctx.bumps.get("market").unwrap();
    let vault_bump = *ctx.bumps.get("vault").unwrap();

    // Store Market configuration
    market.authority = ctx.accounts.authority.key();
    market.token_mint = ctx.accounts.token_mint.key();
    market.vault = ctx.accounts.vault.key();
    market.bump = market_bump;
    market._reserved = [0; 7];

    // Store Vault configuration
    vault.authority = ctx.accounts.vault_authority.key();
    vault.token_account = ctx.accounts.vault_token_account.key();
    vault.token_mint = ctx.accounts.token_mint.key();
    vault.bump = vault_bump;
    vault._reserved = [0; 7];

    Ok(())
}
