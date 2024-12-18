use std::ptr::read;

use crate::{accounts, constants::*};
use crate::{error::ErrorCode, state::StakerSplInfo, SplPoolState};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};
#[derive(Accounts)]
pub struct ClaimSpl<'info> {
    #[account(mut)]
    pub claimer: Signer<'info>,

    #[account(
        mut,
        seeds=[SPL_STAKER_SEED,claimer.key().as_ref()],
        bump,
    )]
    pub claimer_info: Account<'info, StakerSplInfo>,
    #[account(
        init_if_needed, //Ensure that if the account isn't initialize, make it initialize
        payer = claimer,
        associated_token::mint = mint,//Ensure that this TokenAccount reflects the same mint during initialize
        associated_token::authority = claimer,//Ensure that  TokenAccount's authority holder is the staker
        constraint = claimer_token_acc.owner == claimer.key() @ErrorCode ::InvalidUserStakingWalletOwner //Ensure that TokenAccount's owner  is the staker
    )]
    pub claimer_token_acc: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [SPL_POOL_STATE_SEED],
        bump,
    )]
    pub pool_state: Account<'info, SplPoolState>,

    #[account(
         mut,
         seeds = [SPL_REWARD_VAULT_SEED, pool_state.key().as_ref()],
         bump,
         constraint = reward_vault.mint.key() == mint.key() @ErrorCode::InvalidMint,//Ensure that stake vault contains the same mint
         constraint = reward_vault.owner.key() == pool_state.key() @ErrorCode::PdaIsToBeOwner
    )]
    pub reward_vault: Account<'info, TokenAccount>,

    #[account(
        constraint = pool_state.mint.key() == mint.key() @ErrorCode::InvalidMint//Enusre that this mint is same as the mint during initialize
     )]
    pub mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

pub fn claim_spl(ctx: Context<ClaimSpl>) -> Result<()> {
    let accounts = ctx.accounts;
    let current_timestamp = Clock::get().unwrap().unix_timestamp as u64;

    //CHECK that staking duration  has ended
    let staking_end_time = accounts.pool_state.init_timestamp + accounts.pool_state.stake_duration;

    if current_timestamp > staking_end_time {
        // if the currenttime is beyond the staking end time
        require!(
            accounts.claimer_info.last_claimed_time + accounts.pool_state.claim_reward_interval
                <= staking_end_time,
            ErrorCode::StakingDurationEnded
        );
    }
    //Check that user has claimed first time
    if accounts.claimer_info.last_claimed_time > 0 {
        let next_allowed_claim =
            accounts.claimer_info.last_claimed_time + accounts.pool_state.claim_reward_interval;
        //if the current time is less than than the expected claim time so prevent the claimer
        require_gte!(
            current_timestamp,
            next_allowed_claim,
            ErrorCode::ClaimIntervalNotReached
        );
    }

    let reward = accounts
        .claimer_info
        .staker_staked_amount
        .checked_mul(accounts.pool_state.reward_rate)
        .and_then(|v| v.checked_div(100))
        .ok_or(ErrorCode::Overflow)?;

    require_gte!(reward, 0, ErrorCode::NoRewardsToClaim);

    msg!("{}", reward);
    
    accounts.claimer_info.last_claimed_time = current_timestamp;

    let pool_state_authority = &[SPL_POOL_STATE_SEED];
    let (_, pool_state_bump) = Pubkey::find_program_address(pool_state_authority, &ctx.program_id);
    //update the
    let claim_transfer_ix = token::Transfer {
        from: accounts.reward_vault.to_account_info(),
        to: accounts.claimer_token_acc.to_account_info(),
        authority: accounts.pool_state.to_account_info(),
    };
    let pool_state_signer_seeds: &[&[u8]] = &[SPL_POOL_STATE_SEED, &[pool_state_bump]];

    // Create the CPI context for the token transfer
    let signer_bind = [pool_state_signer_seeds];
    let cpi_context = CpiContext::new_with_signer(
        accounts.token_program.to_account_info(),
        claim_transfer_ix,
        &signer_bind,
    );

    token::transfer(cpi_context, reward)?;

    Ok(())
}
