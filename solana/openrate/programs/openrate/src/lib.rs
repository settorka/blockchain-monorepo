#![allow(deprecated)]

use anchor_lang::prelude::*;

pub mod state;
pub mod instructions;
pub mod errors;
use instructions::*;

declare_id!("7baz5aqHG5CUPqpsavhXDgTmHkYgKVthDuuFkT3Fbdt9");

#[program]
pub mod openrate {
    use super::*;

    pub fn initialize_market(ctx: Context<InitializeMarket>) -> Result<()> {
        instructions::initialize_market(ctx)
    }

    pub fn place_bid(ctx: Context<PlaceBid>, amount: u64, rate_bps: u16) -> Result<()> {
        instructions::place_bid(ctx, amount, rate_bps)
    }

    pub fn borrow(ctx: Context<Borrow>, borrow_amount: u64) -> Result<()> {
        instructions::borrow(ctx, borrow_amount)
    }

    pub fn repay(ctx: Context<Repay>) -> Result<()> {
        instructions::repay(ctx)
    }

    pub fn cancel_bid(ctx: Context<CancelBid>) -> Result<()> {
        instructions::cancel_bid(ctx)
    }
}
