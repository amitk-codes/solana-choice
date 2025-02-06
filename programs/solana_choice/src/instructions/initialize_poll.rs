use anchor_lang::prelude::*;

use crate::{PollAccount, ANCHOR_DISCRIMINATOR};

pub fn initialize_poll_handler(
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
        total_number_of_choices: 0,
    });
    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64)]
pub struct InitializePoll<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + PollAccount::INIT_SPACE,
        seeds = [b"poll", poll_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
}
