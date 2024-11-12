use anchor_lang::prelude::*;

declare_id!("8QmoUoRsRFnJLedRhqTUZWxyLAMuWidM8S4TAj7JQACA");

#[program]
pub mod token_stake {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
