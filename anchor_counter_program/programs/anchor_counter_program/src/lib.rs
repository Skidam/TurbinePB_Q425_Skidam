use anchor_lang::prelude::*;

declare_id!("C7kmTo8Bxoq844esi4ppL4mLXoBG71LHvHG4UHvHWqv5");

#[program]
pub mod anchor_counter_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
