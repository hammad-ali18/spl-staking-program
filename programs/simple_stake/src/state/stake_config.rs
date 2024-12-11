use anchor_lang::prelude::*;


#[account]
pub struct StakerSplInfo {
    pub staker_staked_amount: u64,
    pub staked_timestamp:u64,
    pub unclaimed_reward:u64,
    pub last_claimed_time:u64
}



impl StakerSplInfo{
    pub const LEN: usize =8+ (8+8+8+8);
}