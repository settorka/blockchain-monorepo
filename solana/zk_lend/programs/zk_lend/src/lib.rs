use anchor_lang::prelude::*;

declare_id!("5yyTV7F75vyHXn2kWQZ7XxPoZ6GNEzPgyBFjiUF5dXwc");

#[program]
pub mod zk_lend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
