use anchor_lang::prelude::*;

declare_id!("CPttFgMJ1nPU8KDUbBsLWmSQWB1f74R3i8Nt8k59Dgw9");

pub mod instructions;
pub use instructions::*;

pub mod state;
pub use state::*;

pub mod constant;
pub use constant::*;

pub mod error;
pub use error::*;

#[program]
pub mod solana_choice {
    use super::*;

    pub fn initialize_poll(
        ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        start_date: u64,
        end_date: u64,
    ) -> Result<()> {
        initialize_poll_handler(ctx, poll_id, description, start_date, end_date)?;
        Ok(())
    }

    pub fn initialize_choice(
        ctx: Context<InitializeChoice>,
        poll_id: u64,
        choice_name: String,
    ) -> Result<()> {
        initialize_choice_handler(ctx, poll_id, choice_name)?;
        Ok(())
    }

    pub fn vote(ctx: Context<Vote>, poll_id: u64, choice_name: String) -> Result<()> {
        vote_handler(ctx, poll_id, choice_name)?;

        Ok(())
    }
}
