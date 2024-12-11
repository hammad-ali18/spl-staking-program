use crate::constants::*;
use crate::{
    error::ErrorCode,
    state::{ StakerSplInfo},
     SplPoolState,
};
use anchor_lang::{prelude::*};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};
#[derive(Accounts)]
pub struct StakeSpl<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(
        init,
        payer = staker,
        seeds=[SPL_STAKER_SEED,staker.key().as_ref()],
        bump,
        space = StakerSplInfo::LEN
    )]
    pub staker_info: Account<'info, StakerSplInfo>,
    #[account(
        init_if_needed, //Ensure that if the account isn't initialize, make it initialize
        payer = staker,
        associated_token::mint = mint,//Ensure that this TokenAccount reflects the same mint during initialize
        associated_token::authority = staker,//Ensure that  TokenAccount's authority holder is the staker
        constraint = staker_token_acc.owner == staker.key() @ErrorCode ::InvalidUserStakingWalletOwner //Ensure that TokenAccount's owner  is the staker
    )]
    pub staker_token_acc: Account<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [SPL_POOL_STATE_SEED],
        bump,
    )]
    pub pool_state: Account<'info, SplPoolState>,

    #[account(
         mut,
         seeds = [SPL_STAKE_VAULT_SEED, pool_state.key().as_ref()],
         bump,
         constraint = stake_vault.mint.key() == mint.key() @ErrorCode::InvalidMint,//Ensure that stake vault contains the same mint
         constraint = stake_vault.owner.key() == pool_state.key() @ErrorCode::PdaIsToBeOwner
    )]
    pub stake_vault: Account<'info, TokenAccount>,

    #[account(
        constraint = pool_state.mint.key() == mint.key() @ErrorCode::InvalidMint//Enusre that this mint is same as the mint during initialize
     )]
    pub mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,

    pub token_program: Program<'info, Token>,

    pub system_program: Program<'info, System>,
}

pub fn stake_spl(ctx: Context<StakeSpl>, amount: u64) -> Result<()> {
    let accounts = ctx.accounts;
    let staker_info = &mut accounts.staker_info;
    let pool_state = &mut accounts.pool_state;
    let current_timestamp = Clock::get().unwrap().unix_timestamp as u64;

    let staker = &accounts.staker;
    let staker_token_acc = &accounts.staker_token_acc;
    let stake_vault = &accounts.stake_vault;

    // Ensure the staker has enough tokens to stake
    let staker_balance = staker_token_acc.amount;
    require_gt!(staker_balance, amount, ErrorCode::InsufficientFunds);

    // Update the staker's staked info
    staker_info.staked_timestamp = current_timestamp;
    staker_info.staker_staked_amount += amount;
    pool_state.total_spl_staked += amount;


    // Create the CPI context for token transfer
    let cpi_context = CpiContext::new(
        accounts.token_program.to_account_info(),
        token::Transfer {
            from: staker_token_acc.to_account_info(),
            to: stake_vault.to_account_info(),
            authority: staker.to_account_info(),
        },
    );

    // Transfer the SPL tokens from the staker to the vault
    token::transfer(cpi_context, amount)?;
    Ok(())
}
