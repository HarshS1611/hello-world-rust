use anchor_lang::prelude::*;

declare_id!("9gKhdaXwaxohoMVBviDCwmZ46ysv47pPg9UKeGaZoCep");

#[program]
pub mod hello_world_rust {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
