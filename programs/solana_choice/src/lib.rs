use anchor_lang::prelude::*;

declare_id!("CPttFgMJ1nPU8KDUbBsLWmSQWB1f74R3i8Nt8k59Dgw9");

#[program]
pub mod solana_choice {
    use super::*;

    pub fn initialize_poll
    (
        ctx: Context<InitializePoll>,
        poll_id: u64,
        description: String,
        start_date: u64,
        end_date: u64,
    ) -> Result<()> {
        msg!("Poll account is initializing...");

        ctx.accounts.poll_account.set_inner(PollAccount {
            poll_id,
            description,
            start_date,
            end_date,
            total_number_of_choices: 0
        });
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = 8 + PollAccount::INIT_SPACE,
        seeds = [poll_id.to_le_bytes().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    poll_id: u64,

    #[max_len(300)]
    description: String,

    start_date: u64,
    end_date: u64,
    total_number_of_choices: u64,
}
