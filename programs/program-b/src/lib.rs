use anchor_lang::prelude::*;

declare_id!("EknHTqyGbmEKdaxBV9v64NAdhS2NzZY57pjKscRV3E23");

#[program]
pub mod program_b {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from program B");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    pub pda_accoutn: Signer<'info>,
    
}
