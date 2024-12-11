use anchor_lang::prelude::*;

#[constant]

pub const SPL_POOL_STATE_SEED: &[u8;13] = b"init_spl_pool";

pub const SPL_STAKE_VAULT_SEED: &[u8;15]= b"stake_spl_vault";

pub const SPL_REWARD_VAULT_SEED: &[u8;16]=b"reward_spl_vault";

pub const SPL_STAKER_SEED:&[u8;15] =b"staker_spl_info";