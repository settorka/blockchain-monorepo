use anchor_lang::prelude::*;

declare_id!("7baz5aqHG5CUPqpsavhXDgTmHkYgKVthDuuFkT3Fbdt9");

#[program]
pub mod openrate {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
