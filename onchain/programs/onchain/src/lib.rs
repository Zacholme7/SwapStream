use anchor_lang::prelude::*;

declare_id!("Fgywu3A2Ts6yYLbruPkVHKJiA2D9KAz15cAsmhGU7j8F");

#[program]
pub mod onchain {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
