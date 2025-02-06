use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct PollAccount {
    pub poll_id: u64,

    #[max_len(300)]
    pub description: String,

    pub start_date: u64,
    pub end_date: u64,
    pub total_number_of_choices: u64,
}

#[account]
#[derive(InitSpace)]
pub struct ChoiceAccount {
    #[max_len(40)]
    pub choice_name: String,
    pub choice_votes: u64,
}
