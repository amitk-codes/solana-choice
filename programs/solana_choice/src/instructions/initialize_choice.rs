use anchor_lang::prelude::*;

use crate::{ChoiceAccount, PollAccount, ANCHOR_DISCRIMINATOR};

pub fn initialize_choice_handler(
    ctx: Context<InitializeChoice>,
    poll_id: u64,
    choice_name: String,
) -> Result<()> {
    msg!(
        "Initializing the choice {} for poll_id {}",
        choice_name,
        poll_id
    );

    let poll_account = &mut ctx.accounts.poll_account;
    poll_account.total_number_of_choices += 1;

    ctx.accounts.choice_account.set_inner(ChoiceAccount {
        choice_name,
        choice_votes: 0,
    });

    Ok(())
}

#[derive(Accounts)]
#[instruction(poll_id: u64, choice_name: String)]
pub struct InitializeChoice<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds = [b"poll", poll_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    #[account(
        init_if_needed,
        payer = signer,
        space = ANCHOR_DISCRIMINATOR + ChoiceAccount::INIT_SPACE,
        seeds = [b"choice", poll_id.to_le_bytes().as_ref(), choice_name.as_ref(), signer.key().as_ref()],
        bump
    )]
    pub choice_account: Account<'info, ChoiceAccount>,

    pub system_program: Program<'info, System>,
}
