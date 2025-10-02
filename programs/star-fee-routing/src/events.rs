use anchor_lang::prelude::*;

/// Event emitted when honorary position is initialized
#[event]
pub struct HonoraryPositionInitialized {
    /// The position pubkey
    pub position: Pubkey,
    /// The position owner PDA
    pub position_owner: Pubkey,
    /// The vault seed used
    pub vault_seed: u64,
    /// Lower tick of the position
    pub lower_tick: i32,
    /// Upper tick of the position
    pub upper_tick: i32,
    /// Quote mint
    pub quote_mint: Pubkey,
}

/// Event emitted when quote fees are claimed
#[event]
pub struct QuoteFeesClaimed {
    /// Amount of quote fees claimed
    pub amount_claimed: u64,
    /// Quote mint
    pub quote_mint: Pubkey,
    /// Timestamp when claimed
    pub timestamp: i64,
}

/// Event emitted for each investor payout page
#[event]
pub struct InvestorPayoutPage {
    /// Current page index
    pub page_index: u32,
    /// Number of investors in this page
    pub investor_count: u32,
    /// Total amount distributed in this page
    pub total_distributed: u64,
    /// Timestamp of distribution
    pub timestamp: i64,
}

/// Event emitted when creator receives remainder and day is closed
#[event]
pub struct CreatorPayoutDayClosed {
    /// Amount sent to creator
    pub creator_amount: u64,
    /// Total distributed to investors today
    pub total_investor_distributed: u64,
    /// Quote mint
    pub quote_mint: Pubkey,
    /// Timestamp when day closed
    pub timestamp: i64,
}
