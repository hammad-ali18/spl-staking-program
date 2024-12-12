use crate::constants::*;
use crate::{error::ErrorCode, state::StakerSplInfo, SplPoolState};
use anchor_lang::{prelude::*, solana_program::system_instruction, system_program};
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount},
};

#[derive(Accounts)]
pub struct UnstakeSpl<'info> {
    #[account(mut)]
    pub staker: Signer<'info>,

    #[account(
        mut,
        seeds=[SPL_STAKER_SEED,staker.key().as_ref()],
        bump,
    )]
    pub staker_info: Account<'info, StakerSplInfo>,

    #[account(
        mut,
        seeds = [SPL_POOL_STATE_SEED],
        bump,
    )]
    pub pool_state: Account<'info, SplPoolState>,

    #[account(mut,seeds = [SPL_STAKE_VAULT_SEED, pool_state.key().as_ref()], constraint = stake_vault.mint.key() == mint.key() @ErrorCode::InvalidMint, constraint = stake_vault.owner.key() == pool_state.key() @ErrorCode::PdaIsToBeOwner , bump)]
    pub stake_vault: Account<'info, TokenAccount>,
    #[account(mut,
        associated_token::mint = mint,
        associated_token::authority = staker,
        constraint = staker_token_acc.owner == staker.key() @ErrorCode ::InvalidUserStakingWalletOwner
    )]
    pub staker_token_acc: Account<'info, TokenAccount>,

    #[account(constraint = pool_state.mint.key() == mint.key() @ErrorCode::InvalidMint )]
    pub mint: Account<'info, Mint>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn unstake_spl(ctx: Context<UnstakeSpl>, amount: u64) -> Result<()> {
    let accounts = ctx.accounts;
    let staker_info = &mut accounts.staker_info;
    let pool_state = &mut accounts.pool_state;

    let staker = &accounts.staker;
    let staker_token_acc = &accounts.staker_token_acc;
    let stake_vault = &accounts.stake_vault;

    // Ensure the staker has staked enough tokens to unstake
    require_gte!(
        staker_info.staker_staked_amount,
        amount,
        ErrorCode::InsufficientStakedAmount
    );

    // Update staker info and pool state
    staker_info.staker_staked_amount -= amount;
    pool_state.total_spl_staked -= amount;

    let transfer_instruction = token::Transfer {
        from: stake_vault.to_account_info(),
        to: staker_token_acc.to_account_info(),
        authority: pool_state.to_account_info(),
    };


    // Derive the pool_state authority (PDA) using the correct seeds and bump
    // let pool_state_bind = pool_state.key();
// Derive the PDA for the pool state using the correct seeds
let pool_state_authority = &[SPL_POOL_STATE_SEED];
let (pool_state_pda, pool_state_bump) = Pubkey::find_program_address(pool_state_authority, &ctx.program_id);


require_eq!(
    pool_state_pda,
    pool_state.key(),
    ErrorCode::InvalidAuthority
);

// Define the signer seeds with the bump included
let pool_state_signer_seeds: &[&[u8]] = &[SPL_POOL_STATE_SEED, &[pool_state_bump]];

// Create the CPI context for the token transfer
let binding = [pool_state_signer_seeds];
let cpi_context = CpiContext::new_with_signer(
    accounts.token_program.to_account_info(),
    transfer_instruction,
    &binding,
);

// Transfer the SPL tokens from the stake vault back to the staker's account
token::transfer(cpi_context, amount)?;


    Ok(())
}
