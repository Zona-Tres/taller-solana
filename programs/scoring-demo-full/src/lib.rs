use anchor_lang::prelude::*;

declare_id!("7YPt7L7A7zZvBufGJ59pzWhcPgeVZgUVefi86nCs3jVa");

#[program]
pub mod scoring_demo_full {
    use super::*;

    pub fn open_scoring(ctx: Context<OpenScoringInstruction>, name: String) -> Result<()> {
        // Validate input.

        // Create scoring account.
        ctx.accounts.scoring_account.set_inner(ScoringAccount {
            name,
            creator: ctx.accounts.creator.key(),
            highest_vote: 0,
            high_vote: 0,
            low_vote: 0,
            lowest_vote: 0,
            total_votes: 0,
        });

        Ok(())
    }

    pub fn vote(ctx: Context<VoteInstruction>, vote: ScoringVote) -> Result<()> {
        // Validate input.

        // Modify scoring account.
        let scoring_account = &mut ctx.accounts.scoring_account;
        scoring_account.total_votes += 1;

        match vote {
            ScoringVote::Highest => scoring_account.highest_vote += 1,
            ScoringVote::High => scoring_account.high_vote += 1,
            ScoringVote::Low => scoring_account.low_vote += 1,
            ScoringVote::Lowest => scoring_account.lowest_vote += 1,
        }

        // Create vote account.
        ctx.accounts.vote_account.set_inner(VoteAccount {
            scoring_account: ctx.accounts.scoring_account.key(),
            voter: ctx.accounts.voter.key(),
            vote,
        });

        Ok(())
    }

    pub fn edit_vote(ctx: Context<EditVoteInstruction>, new_vote: ScoringVote) -> Result<()> {
        // Validate input.

        // Modify scoring account.
        let scoring_account = &mut ctx.accounts.scoring_account;

        match new_vote {
            ScoringVote::Highest => scoring_account.highest_vote += 1,
            ScoringVote::High => scoring_account.high_vote += 1,
            ScoringVote::Low => scoring_account.low_vote += 1,
            ScoringVote::Lowest => scoring_account.lowest_vote += 1,
        }

        let old_vote = ctx.accounts.vote_account.vote;
        match old_vote {
            ScoringVote::Highest => scoring_account.highest_vote -= 1,
            ScoringVote::High => scoring_account.high_vote -= 1,
            ScoringVote::Low => scoring_account.low_vote -= 1,
            ScoringVote::Lowest => scoring_account.lowest_vote -= 1,
        }

        Ok(())
    }
}

// ---- Instruction arguments ----

#[derive(Accounts)]
#[instruction(name: String)]
pub struct OpenScoringInstruction<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 8 * 5 + 32 + 4 + name.len(),
        seeds = [name.as_ref()],
        bump
    )]
    pub scoring_account: Account<'info, ScoringAccount>,

    pub creator: Signer<'info>,

    /// CHECK: Account payer.
    #[account(mut)]
    pub payer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct VoteInstruction<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 32 * 2 + 1,
        seeds = [scoring_account.key().as_ref(), voter.key().as_ref()],
        bump
    )]
    pub vote_account: Account<'info, VoteAccount>,

    #[account(mut)]
    pub scoring_account: Account<'info, ScoringAccount>,

    pub voter: Signer<'info>,

    /// CHECK: Account payer.
    #[account(mut)]
    pub payer: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EditVoteInstruction<'info> {
    #[account(mut)]
    #[account(has_one = scoring_account)]
    #[account(has_one = voter)]
    pub vote_account: Account<'info, VoteAccount>,

    #[account(mut)]
    pub scoring_account: Account<'info, ScoringAccount>,

    pub voter: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Copy, Clone, AnchorSerialize, AnchorDeserialize)]
#[repr(u8)]
pub enum ScoringVote {
    Highest,
    High,
    Low,
    Lowest,
}

// ---- Accounts ----

#[account]
pub struct ScoringAccount {
    pub name: String,
    pub creator: Pubkey,
    pub highest_vote: u64,
    pub high_vote: u64,
    pub low_vote: u64,
    pub lowest_vote: u64,
    pub total_votes: u64,
}

#[account]
pub struct VoteAccount {
    pub scoring_account: Pubkey,
    pub voter: Pubkey,
    pub vote: ScoringVote,
}
