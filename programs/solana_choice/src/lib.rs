use anchor_lang::prelude::*;

declare_id!("CPttFgMJ1nPU8KDUbBsLWmSQWB1f74R3i8Nt8k59Dgw9");

#[program]
pub mod solana_choice {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
