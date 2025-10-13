    use anchor_lang::prelude::*;
    use anchor_spl::token::{self, Token, TokenAccount, Transfer};
    use crate::state::*;
    use crate::errors::ErrorCode;


    /// Borrow instruction
    ///
    /// Allows a borrower to take a loan from the market vault by
    /// matching against one or more existing bid offers. The borrowed
    /// amount is transferred from the vault to the borrower’s SPL
    /// token account, and a new BorrowRecord is created to track
    /// the position and repayment terms.
    ///
    /// this instruction assumes the borrower is
    /// borrowing against a single matching BidOrder (the lowest-rate
    /// available bid).
    #[derive(Accounts)]
    pub struct Borrow<'info> {
        /// The borrower initiating the loan.
        #[account(mut, signer)]
        pub borrower: Signer<'info>,

        /// The market from which funds are borrowed.
        #[account(mut, has_one = vault)]
        pub market: Account<'info, Market>,

        /// The vault metadata for this market.
        #[account(mut)]
        pub vault: Account<'info, Vault>,

        /// The actual SPL token account holding market liquidity.
        #[account(mut, constraint = vault_token_account.key() == vault.token_account)]
        pub vault_token_account: Account<'info, TokenAccount>,

        /// The borrower’s SPL token account that will receive the funds.
        #[account(mut, constraint = borrower_token_account.mint == market.token_mint)]
        pub borrower_token_account: Account<'info, TokenAccount>,

        /// The matched bid order being filled by this borrow.
        #[account(mut, has_one = market, constraint = bid_order.is_active == true)]
        pub bid_order: Account<'info, BidOrder>,

        /// The new borrow record account to be created.
        #[account(
            init,
            payer = borrower,
            space = BorrowRecord::LEN,
            seeds = [b"borrow_record", borrower.key().as_ref(), bid_order.key().as_ref()],
            bump
        )]
        pub borrow_record: Account<'info, BorrowRecord>,

        /// PDA that owns the vault; acts as transfer authority.
        #[account(seeds = [b"vault_authority", market.key().as_ref()], bump = vault.bump)]
        pub vault_authority: SystemAccount<'info>,

        /// System and token programs.
        pub system_program: Program<'info, System>,
        pub token_program: Program<'info, Token>,
        pub rent: Sysvar<'info, Rent>,
    }

    /// Borrow instruction handler
    ///
    /// Transfers tokens from the vault to the borrower, updates the
    /// BidOrder to reflect filled status, and creates a new
    /// BorrowRecord capturing loan details and rate information.
    pub fn borrow(ctx: Context<Borrow>, borrow_amount: u64) -> Result<()> {
        let vault = &ctx.accounts.vault;
        let bid_order = &mut ctx.accounts.bid_order;
        let borrow_record = &mut ctx.accounts.borrow_record;
        let borrower = &ctx.accounts.borrower;
        let market = &ctx.accounts.market;
        let clock = Clock::get()?;

        require!(bid_order.is_active, ErrorCode::BidInactive);
        require!(borrow_amount <= bid_order.amount - bid_order.filled_amount, ErrorCode::InsufficientBidLiquidity);

        // Transfer funds from vault to borrower.
        let market_key = market.key();
        let bump = vault.bump;
        let seeds: &[&[u8]] = &[b"vault_authority", market_key.as_ref(), &[bump]];

        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.borrower_token_account.to_account_info(),
            authority: ctx.accounts.vault_authority.to_account_info(),
        };
        let cpi_ctx = CpiContext::new_with_signer(ctx.accounts.token_program.to_account_info(), cpi_accounts, signer_seeds);
        token::transfer(cpi_ctx, borrow_amount)?;

        // Update bid order fill.
        bid_order.filled_amount += borrow_amount;
        if bid_order.filled_amount >= bid_order.amount {
            bid_order.is_active = false;
        }

        // Create borrow record.
        let borrow_bump = ctx.bumps.borrow_record;
        borrow_record.borrower = borrower.key();
        borrow_record.market = market.key();
        borrow_record.principal = borrow_amount;
        borrow_record.rate_bps = bid_order.rate_bps;
        borrow_record.start_time = clock.unix_timestamp;
        borrow_record.repaid = false;
        borrow_record.bump = borrow_bump;
        borrow_record._reserved = [0; 6];

        Ok(())
    }
