use anchor_lang::prelude::*;

declare_id!("CPttFgMJ1nPU8KDUbBsLWmSQWB1f74R3i8Nt8k59Dgw9");

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

    pub fn initialize_choice(
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
        seeds = [b"poll", poll_id.to_le_bytes().as_ref(), signer.key().as_ref()],
        bump
    )]
    pub poll_account: Account<'info, PollAccount>,

    pub system_program: Program<'info, System>,
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
        space = 8 + ChoiceAccount::INIT_SPACE,
        seeds = [b"choice", poll_id.to_le_bytes().as_ref(), choice_name.as_ref(), signer.key().as_ref()],
        bump
    )]
    pub choice_account: Account<'info, ChoiceAccount>,

    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
#[instruction(poll_id: u64, choice_name: String)]
pub struct Vote <'info> {
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

#[account]
#[derive(InitSpace)]
pub struct ChoiceAccount {
    #[max_len(40)]
    choice_name: String,
    choice_votes: u64,
}
