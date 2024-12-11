use anchor_lang::{prelude::*};
use anchor_spl::token::{Mint, Token, TokenAccount};
use crate::state::initialize_config::{SplPoolState};
use crate::error::{ErrorCode};
use crate::constants::*;

#[derive(Accounts)]
pub struct InitializeSplPool<'info> {
    /// **Pool State Account PDA**
    /// - `init`: Indicates that this account will be initialized.
    /// - `seeds`: A set of deterministic inputs used to derive the Program Derived Address (PDA). 
    ///   The resulting PDA is unique to the program and the provided seeds, ensuring the address can be recreated consistently.
    ///   The seed value `b"init_sol_pool"` is used here.
    /// - `payer`: Specifies the account (in this case, `initializer`) that will cover the costs of creating the new account.
    /// - `space`: Defines the size of the account, ensuring that sufficient storage is allocated for the `SolPoolState` struct.
    /// - `bump`: A bump seed that ensures the derived address is valid and prevents address collisions when generating the PDA.
    /// 
    /// For more details, refer to the official Solana documentation on PDAs: https://solana.com/docs/core/pda#create-pda-accounts

    #[account(
        init,    // Marks the  pool_state account for initialization.                   
        seeds = [SPL_POOL_STATE_SEED],// Seed
        payer = initializer,    // The `initializer` pays for the account creation.      
        space = SplPoolState::LEN,  
        bump        // Bump seed to ensure the PDA is valid and prevents address conflicts.                  
    )]
    pub pool_state: Account<'info, SplPoolState>, 

  
    /// - `[account(mut)]`: The account will be mutated in the transaction, as it may lose some SOL during the account creation.
    /// - `initializer`: This account must sign the transaction and pay for the pool state initialization.
    #[account(mut)] // Indicates that this account will be mutated (its balance will be changed).
    pub initializer: Signer<'info>, // Signer for the transaction (payer for the account creation).

    #[account(
        init,                        
        payer = initializer,        
        seeds = [SPL_STAKE_VAULT_SEED, pool_state.key().as_ref()],  
        bump,                       
        token::mint = mint,         // Ensures the vault is tied to the specified mint.
        token::authority = pool_state // Pool state is the authority for managing the vault.
    )]
    pub stake_vault: Account<'info, TokenAccount>, 

    #[account(
        init,
        payer = initializer,
        seeds = [SPL_REWARD_VAULT_SEED,pool_state.key().as_ref()],
        bump,
        token::mint = mint,
        token::authority = pool_state
    )]
    pub reward_vault : Account<'info,TokenAccount>,

    /// **Mint Account**
    /// - The SPL token mint account, which defines the token’s type and minting authority.
    /// - This account must exist for the pool to be associated with the token, ensuring that the program can mint tokens.
    ///   In this case, the account must implement the `Mint` interface of the SPL Token Program, which manages token minting operations.

    pub mint: Account<'info, Mint>, // The mint that will issue tokens for the pool.

    /// **Token Program**
    /// - Represents the SPL Token Program which is used to manage token accounts, transfers, and minting.
    /// - This is required to perform token operations (e.g., minting tokens, transferring tokens) in the context of the pool.
    pub token_program: Program<'info, Token>, // The SPL Token Program to handle token-related operations.

    
    /// **System Program**
    /// - Refers to the native Solana `SystemProgram`, which allows for system-level operations such as account creation and transfers.
    /// - The `system_program` is necessary to interact with Solana's underlying account management system.
    /// For more details, refer to the official Solana documentation on system-program: https://solana.com/docs/core/accounts#system-program
    pub system_program: Program<'info, System>, // Solana's built-in program for creating accounts and managing SOL transfers.
    

}

pub fn spl_handler(ctx: Context<InitializeSplPool>,pool_owner:Pubkey,reward_rate:u64,stake_duration:u64,reward_interval_duration:u64) -> Result<()> {

    let accounts =  ctx.accounts;  
    let pool_state = &mut accounts.pool_state;
    let current_timestamp = Clock::get().unwrap().unix_timestamp as u64;

    require_gt!(stake_duration,0, ErrorCode::InvalidTimeStamp);

    pool_state.pool_authority  = pool_owner;
    pool_state.total_spl_staked =0;
    pool_state.reward_rate = reward_rate;
    pool_state.mint = accounts.mint.key();

    pool_state.init_timestamp = current_timestamp;
    pool_state.stake_duration = stake_duration;
    pool_state.claim_reward_interval = reward_interval_duration;

    Ok(())
}
