use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The provided staking wallet does not belong to the user.")]
    InvalidUserStakingWalletOwner,
    #[msg("The provided staking wallet does not belong to the admin.")]
    InvalidAdminStakingWalletOwner,

    #[msg("Staking Duration should be greater than 0")]
    InvalidStakingDurtaion,

    #[msg("Unable to Stake: StakingDuration Ends")]
    UnableToStake,

    #[msg("Unable to UnStake: User Staking is not Ended")]
    UnableToUnStake,


    #[msg("No Enough Funds")]
    InsufficientFunds,
    #[msg("Tranfer Failed")]
    TransferFailed,
    Overflow,
    #[msg("Unable to stake with this Token")]
    InvalidMint,
    #[msg("Unable to stake with this Token")]
    PdaIsToBeOwner,
    
    #[msg("Insufficient Staked amount")]
    InsufficientStakedAmount,

    #[msg("InvalidTimeStamp")]
    InvalidTimeStamp,

    #[msg("7 days of claim is not being completed")]
    ClaimIntervalNotReached,
    #[msg("No Rewards to claim")]
     NoRewardsToClaim,

     StakingDurationEnded,
     InvalidAuthority
}
