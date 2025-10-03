use anchor_lang::prelude::*;

#[error_code]
pub enum FeeRoutingError {
    #[msg("Pool configuration would result in base token fees, which is not allowed")]
    BaseFeeDetected,

    #[msg("Distribution can only be called once per 24 hour period")]
    TooEarlyForDistribution,

    #[msg("Invalid quote mint - pool token order validation failed")]
    InvalidQuoteMint,

    #[msg("No fees available to claim")]
    NoFeesAvailable,

    #[msg("Base fees detected during claim - distribution aborted")]
    BaseFeesClaimedError,

    #[msg("Arithmetic overflow")]
    ArithmeticOverflow,

    #[msg("Invalid page index")]
    InvalidPageIndex,

    #[msg("Distribution already complete for this day")]
    DistributionAlreadyComplete,

    #[msg("Insufficient locked tokens")]
    InsufficientLockedTokens,

    #[msg("Invalid investor data")]
    InvalidInvestorData,

    #[msg("Daily cap exceeded")]
    DailyCapExceeded,

    #[msg("Payout below minimum threshold")]
    PayoutBelowThreshold,

    #[msg("Invalid tick range for quote-only position")]
    InvalidTickRange,

    #[msg("Position not owned by program PDA")]
    InvalidPositionOwner,

    #[msg("Invalid Streamflow contract data - unable to deserialize")]
    InvalidStreamflowContract,
}
