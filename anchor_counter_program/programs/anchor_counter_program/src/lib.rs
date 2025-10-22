use anchor_lang::prelude::*;

declare_id!("C7kmTo8Bxoq844esi4ppL4mLXoBG71LHvHG4UHvHWqv5");

#[program]
pub mod anchor_counter_program {
    use super::*;

    pub fn hello(_ctx: Context<Hello>) -> Result<()> {

        msg!("Hello World"); 

        msg!("Our program's Program ID: {}", &id());

        let mut counter = 0; 

        let result = loop {
            counter += 1; 
            msg!("Current Count is: {counter}");
            

            if counter >= 10 {
                break counter; 
            }
        };

        msg!("The Final Count is {result}");

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Hello{}
