use anchor_lang::prelude::*;

declare_id!("AaCpz71iBEGMzsx2nbYxj12s9apVuvr7iiFdFmMeVuJ6");

#[program]
pub mod solana_hello_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Hello Sol, we're here to Increase Bandwith and Reduce Latency!!");
        msg!("Special Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
