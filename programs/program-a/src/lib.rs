use anchor_lang::prelude::*;

declare_id!("9meeySgNgNmG29VYBNjmdqfEaAMpgYb63DB8SBe8v6vX");

#[program]
pub mod program_a {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
