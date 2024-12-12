pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;
use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("3hN2cwQpZ4Ymg8FEbeEQM73Ek9W4FNNFRpxtuc2UkqU7");
#[program]
pub mod simple_stake {
    use super::*;

    pub fn initialize_spl_pool(
        ctx: Context<InitializeSplPool>,
        pool_owner: Pubkey,
        reward_rate: u64,
        stake_duration: u64,
        reward_interval: u64,
    ) -> Result<()> {
        initialize::spl_handler(
            ctx,
            pool_owner,
            reward_rate,
            stake_duration,
            reward_interval,
        )?;
        Ok(())
    }

    pub fn stake_spl(ctx: Context<StakeSpl>, amount: u64) -> Result<()> {
        stake::stake_spl(ctx, amount)?;
        Ok(())
    }

    pub fn unstake_spl(ctx: Context<UnstakeSpl>, amount: u64) -> Result<()> {
        unstake::unstake_spl(ctx, amount)?;
        Ok(())
    }

    pub fn claim_spl(ctx: Context<ClaimSpl>) -> Result<()> {
        claim::claim_spl(ctx)?;
        Ok(())
    }
}
