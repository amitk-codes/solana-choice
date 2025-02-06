use anchor_lang::prelude::*;

use crate::{ChoiceAccount, CustomErrors, PollAccount};

pub fn vote_handler(ctx: Context<Vote>, _poll_id: u64, _choice_name: String) -> Result<()> {
    msg!("Vote instruction is being called...");

    let clock = Clock::get()?.unix_timestamp;

    if clock < (ctx.accounts.poll_account.start_date as i64) {
        return Err(CustomErrors::VotingNotStarted.into());
    }

    if clock > (ctx.accounts.poll_account.end_date as i64) {
        return Err(CustomErrors::VotingEnded.into());
    }

    msg!("Voting...");
    let choice_account = &mut ctx.accounts.choice_account;
    choice_account.choice_votes += 1;

    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64, choice_name: String)]
pub struct Vote<'info> {
    pub signer: Signer<'info>,

    #[account(
        seeds = [b"poll", poll_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        mut,
        seeds = [b"choice", poll_id.to_le_bytes().as_ref(), choice_name.as_ref(), signer.key().as_ref()],
        bump
    )]
    pub choice_account: Account<'info, ChoiceAccount>,
}
