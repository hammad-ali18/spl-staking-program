use anchor_lang::prelude::*;

#[account]
pub struct SplPoolState {
    pub pool_authority: Pubkey,
    pub init_timestamp: u64, 
    pub stake_duration:u64,
    pub claim_reward_interval:u64,
    pub mint: Pubkey,
    pub total_spl_staked: u64,
    pub reward_rate: u64,
    pub bump: u8,
}

#[account]
pub struct VaultState {
    pub total_tokens: u64,
    pub owner: Pubkey,
    pub created_at: i64,
    pub token_mint: Pubkey,
    pub status: u8,
}

// The first 8 bytes are the account discriminator.
// Learn more about it here: https://solanacookbook.com/guides/anchor/accounts.html#account-discriminator

impl SplPoolState {
    pub const LEN: usize = 8 + (32 + 8 +8+8+ 32 + 8 + 8 + 1);
}

impl VaultState {
    pub const LEN: usize = 8 + (8 + 32 + 8 + 32 + 1);
}
