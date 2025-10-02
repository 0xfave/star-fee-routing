use anchor_lang::prelude::*;

/// Global state for the fee routing program
#[account]
pub struct GlobalState {
    /// The creator's quote token ATA to receive remaining fees
    pub creator_quote_ata: Pubkey,
    /// Bump seed for the global state PDA
    pub bump: u8,
}

impl GlobalState {
    pub const LEN: usize = 8 + 32 + 1; // discriminator + pubkey + bump
}

/// Distribution progress tracking for the 24h crank
#[account]
pub struct DistributionProgress {
    /// Last distribution timestamp (unix timestamp)
    pub last_distribution_ts: i64,
    /// Total quote fees distributed today
    pub daily_distributed: u64,
    /// Carried over amount from previous distributions (dust)
    pub carry_over: u64,
    /// Current page index for pagination
    pub page_cursor: u32,
    /// Whether the current day's distribution is complete
    pub day_complete: bool,
    /// Vault seed for this distribution
    pub vault_seed: u64,
    /// Bump seed for the PDA
    pub bump: u8,
}

impl DistributionProgress {
    pub const LEN: usize = 8 + 8 + 8 + 8 + 4 + 1 + 8 + 1; // discriminator + fields + bump
}

/// Policy configuration for fee distribution
#[account]
pub struct PolicyConfig {
    /// Fee share for investors in basis points (out of 10000)
    pub investor_fee_share_bps: u16,
    /// Optional daily cap in lamports
    pub daily_cap_lamports: Option<u64>,
    /// Minimum payout threshold in lamports
    pub min_payout_lamports: u64,
    /// Total investor allocation at TGE (Y0)
    pub y0_total: u64,
    /// Vault seed
    pub vault_seed: u64,
    /// Bump seed for the PDA
    pub bump: u8,
}

impl PolicyConfig {
    pub const LEN: usize = 8 + 2 + 9 + 8 + 8 + 8 + 1; // discriminator + fields + bump
}

/// Investor data for fee distribution
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct InvestorData {
    /// Streamflow stream pubkey for this investor
    pub stream_pubkey: Pubkey,
    /// Investor's quote token ATA
    pub investor_quote_ata: Pubkey,
}

impl InvestorData {
    pub const LEN: usize = 32 + 32; // stream_pubkey + investor_quote_ata
}

/// Seeds for PDAs
pub const GLOBAL_STATE_SEED: &[u8] = b"global_state";
pub const VAULT_SEED: &[u8] = b"vault";
pub const INVESTOR_FEE_POSITION_OWNER_SEED: &[u8] = b"investor_fee_pos_owner";
pub const DISTRIBUTION_PROGRESS_SEED: &[u8] = b"distribution_progress";
pub const POLICY_CONFIG_SEED: &[u8] = b"policy_config";
pub const QUOTE_TREASURY_SEED: &[u8] = b"quote_treasury";
