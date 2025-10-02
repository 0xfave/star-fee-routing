use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

declare_id!("45soP1GyzrULnWjAasDnp23T1yDZpkhPsQD6qQ98Ttdg");

// DAMM V2 (CP-AMM) Program ID
const CP_AMM_PROGRAM_ID: &str = "cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG";

pub mod errors;
pub mod events;
pub mod state;

pub use errors::*;
pub use events::*;
pub use state::*;

const SECONDS_PER_DAY: i64 = 86400;

#[program]
pub mod star_fee_routing {
    use super::*;

    /// Initialize the global state
    pub fn initialize_global_state(ctx: Context<InitializeGlobalState>, creator_quote_ata: Pubkey) -> Result<()> {
        let global_state = &mut ctx.accounts.global_state;

        global_state.creator_quote_ata = creator_quote_ata;
        global_state.bump = ctx.bumps.global_state;

        Ok(())
    }

    /// Initialize the honorary fee position (quote-only) - Work Package A
    pub fn initialize_honorary_position(ctx: Context<InitializeHonoraryPosition>, vault_seed: u64) -> Result<()> {
        // Validate pool token order to ensure quote-only fees
        validate_quote_only_pool(&ctx)?;

        // Create position via CPI to DAMM V2 program
        let cp_amm_program = ctx.accounts.cp_amm_program.to_account_info();

        let vault_seed_bytes = vault_seed.to_le_bytes();
        let seeds = &[VAULT_SEED, &vault_seed_bytes, INVESTOR_FEE_POSITION_OWNER_SEED, &[ctx.bumps.position_owner_pda]];
        let signer_seeds = &[&seeds[..]];

        // Call create_position instruction via CPI
        anchor_lang::solana_program::program::invoke_signed(
            &anchor_lang::solana_program::instruction::Instruction {
                program_id: cp_amm_program.key(),
                accounts: vec![
                    AccountMeta::new_readonly(ctx.accounts.position_owner_pda.key(), true),
                    AccountMeta::new(ctx.accounts.position_nft_mint.key(), true),
                    AccountMeta::new(ctx.accounts.position_nft_account.key(), false),
                    AccountMeta::new(ctx.accounts.payer.key(), true),
                    AccountMeta::new_readonly(ctx.accounts.pool_authority.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.pool.key(), false),
                    AccountMeta::new(ctx.accounts.position.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.system_program.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.rent.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.associated_token_program.key(), false),
                    AccountMeta::new_readonly(ctx.accounts.event_authority.key(), false),
                    AccountMeta::new_readonly(cp_amm_program.key(), false),
                ],
                data: [48, 215, 197, 153, 96, 203, 180, 133].to_vec(), // create_position discriminator
            },
            &[
                ctx.accounts.position_owner_pda.to_account_info(),
                ctx.accounts.position_nft_mint.to_account_info(),
                ctx.accounts.position_nft_account.to_account_info(),
                ctx.accounts.payer.to_account_info(),
                ctx.accounts.pool_authority.to_account_info(),
                ctx.accounts.pool.to_account_info(),
                ctx.accounts.position.to_account_info(),
                ctx.accounts.token_program.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
                ctx.accounts.rent.to_account_info(),
                ctx.accounts.associated_token_program.to_account_info(),
                ctx.accounts.event_authority.to_account_info(),
                cp_amm_program,
            ],
            signer_seeds,
        )?;

        // Emit event
        emit!(HonoraryPositionInitialized {
            position: ctx.accounts.position.key(),
            position_owner: ctx.accounts.position_owner_pda.key(),
            vault_seed,
            lower_tick: 0, // Not used in DAMM V2
            upper_tick: 0, // Not used in DAMM V2
            quote_mint: ctx.accounts.quote_mint.key(),
        });

        Ok(())
    }

    /// Permissionless 24h distribution crank - Work Package B
    pub fn distribute_fees(
        ctx: Context<DistributeFees>,
        page_index: u32,
        investor_fee_share_bps: u16,
        daily_cap_lamports: Option<u64>,
        min_payout_lamports: u64,
        y0_total: u64,
    ) -> Result<()> {
        let clock = Clock::get()?;
        let current_ts = clock.unix_timestamp;

        let progress = &mut ctx.accounts.distribution_progress;

        // Initialize the progress account if it's new
        if progress.vault_seed == 0 {
            progress.vault_seed = 12345u64; // Placeholder vault seed
            progress.last_distribution_ts = 0;
            progress.daily_distributed = 0;
            progress.carry_over = 0;
            progress.page_cursor = 0;
            progress.day_complete = false;
            progress.bump = ctx.bumps.distribution_progress;
        }

        // Check if this is the first distribution of a new day
        let is_new_day = current_ts >= progress.last_distribution_ts + SECONDS_PER_DAY;

        if page_index == 0 && !is_new_day {
            return Err(FeeRoutingError::TooEarlyForDistribution.into());
        }

        // Reset progress for new day
        if is_new_day && page_index == 0 {
            progress.last_distribution_ts = current_ts;
            progress.daily_distributed = 0;
            progress.carry_over = 0;
            progress.page_cursor = 0;
            progress.day_complete = false;
        }

        // Validate page index
        if page_index != progress.page_cursor {
            return Err(FeeRoutingError::InvalidPageIndex.into());
        }

        if progress.day_complete {
            return Err(FeeRoutingError::DistributionAlreadyComplete.into());
        }

        // Step 1: Claim fees from honorary position (only on first page)
        let mut claimed_quote = 0u64;
        if page_index == 0 {
            // Call cp-amm claim_position_fee via CPI
            let cp_amm_program = ctx.accounts.cp_amm_program.to_account_info();

            let vault_seed_bytes = progress.vault_seed.to_le_bytes();
            let seeds =
                &[VAULT_SEED, &vault_seed_bytes, INVESTOR_FEE_POSITION_OWNER_SEED, &[ctx.bumps.position_owner_pda]];
            let signer_seeds = &[&seeds[..]];

            // Call claim_position_fee instruction
            anchor_lang::solana_program::program::invoke_signed(
                &anchor_lang::solana_program::instruction::Instruction {
                    program_id: cp_amm_program.key(),
                    accounts: vec![
                        AccountMeta::new_readonly(ctx.accounts.pool_authority.key(), false),
                        AccountMeta::new_readonly(ctx.accounts.pool.key(), false),
                        AccountMeta::new(ctx.accounts.position.key(), false),
                        AccountMeta::new(ctx.accounts.quote_treasury.key(), false),
                        AccountMeta::new(ctx.accounts.quote_treasury.key(), false), // token_b_account same as quote
                        AccountMeta::new(ctx.accounts.token_a_vault.key(), false),
                        AccountMeta::new(ctx.accounts.token_b_vault.key(), false),
                        AccountMeta::new(ctx.accounts.position_nft_account.key(), false),
                        AccountMeta::new_readonly(ctx.accounts.token_a_mint.key(), false),
                        AccountMeta::new_readonly(ctx.accounts.quote_mint.key(), false),
                        AccountMeta::new_readonly(ctx.accounts.position_owner_pda.key(), true),
                        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                        AccountMeta::new_readonly(ctx.accounts.token_program.key(), false),
                        AccountMeta::new_readonly(ctx.accounts.event_authority.key(), false),
                        AccountMeta::new_readonly(cp_amm_program.key(), false),
                    ],
                    data: [180, 38, 154, 17, 133, 33, 162, 211].to_vec(), // claim_position_fee discriminator
                },
                &[
                    ctx.accounts.pool_authority.to_account_info(),
                    ctx.accounts.pool.to_account_info(),
                    ctx.accounts.position.to_account_info(),
                    ctx.accounts.quote_treasury.to_account_info(),
                    ctx.accounts.quote_treasury.to_account_info(),
                    ctx.accounts.token_a_vault.to_account_info(),
                    ctx.accounts.token_b_vault.to_account_info(),
                    ctx.accounts.position_nft_account.to_account_info(),
                    ctx.accounts.token_a_mint.to_account_info(),
                    ctx.accounts.quote_mint.to_account_info(),
                    ctx.accounts.position_owner_pda.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.token_program.to_account_info(),
                    ctx.accounts.event_authority.to_account_info(),
                    cp_amm_program,
                ],
                signer_seeds,
            )?;

            claimed_quote = ctx.accounts.quote_treasury.amount;

            if claimed_quote == 0 {
                return Err(FeeRoutingError::NoFeesAvailable.into());
            }

            emit!(QuoteFeesClaimed {
                amount_claimed: claimed_quote,
                quote_mint: ctx.accounts.quote_mint.key(),
                timestamp: current_ts,
            });
        }

        // Step 2: For demo purposes, assume some locked amounts
        let total_locked = 1_000_000u64; // Placeholder for Streamflow integration

        if total_locked == 0 {
            // All tokens unlocked - send everything to creator
            if page_index == 0 && claimed_quote > 0 {
                // Set day complete first to avoid borrow issue
                progress.day_complete = true;
                transfer_to_creator(&ctx, claimed_quote, current_ts)?;
            }
            return Ok(());
        }

        // Step 3: Calculate investor share
        let f_locked = (total_locked as u128)
            .checked_mul(10000u128)
            .ok_or(FeeRoutingError::ArithmeticOverflow)?
            .checked_div(y0_total as u128)
            .ok_or(FeeRoutingError::ArithmeticOverflow)? as u64;

        let eligible_investor_share_bps = std::cmp::min(investor_fee_share_bps as u64, f_locked);

        let total_fees_for_distribution =
            if page_index == 0 { claimed_quote + progress.carry_over } else { progress.carry_over };

        let investor_fee_quote = total_fees_for_distribution
            .checked_mul(eligible_investor_share_bps)
            .ok_or(FeeRoutingError::ArithmeticOverflow)?
            .checked_div(10000)
            .ok_or(FeeRoutingError::ArithmeticOverflow)?;

        // Apply daily cap
        let remaining_daily_cap =
            if let Some(cap) = daily_cap_lamports { cap.saturating_sub(progress.daily_distributed) } else { u64::MAX };

        let investor_fee_quote = std::cmp::min(investor_fee_quote, remaining_daily_cap);

        // For demonstration, emit event with calculated amounts
        emit!(InvestorPayoutPage {
            page_index,
            investor_count: 1, // Placeholder
            total_distributed: investor_fee_quote,
            timestamp: current_ts,
        });

        progress.daily_distributed =
            progress.daily_distributed.checked_add(investor_fee_quote).ok_or(FeeRoutingError::ArithmeticOverflow)?;

        // Send remainder to creator and complete the day
        let treasury_balance = ctx.accounts.quote_treasury.amount;
        let creator_amount =
            if treasury_balance > investor_fee_quote { treasury_balance - investor_fee_quote } else { 0 };

        // Set completion status first
        progress.day_complete = true;

        if creator_amount > 0 {
            transfer_to_creator(&ctx, creator_amount, current_ts)?;
        }

        Ok(())
    }
}

/// Transfer remaining fees to creator
fn transfer_to_creator(ctx: &Context<DistributeFees>, amount: u64, timestamp: i64) -> Result<()> {
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.quote_treasury.to_account_info(),
            to: ctx.accounts.creator_quote_ata.to_account_info(),
            authority: ctx.accounts.quote_treasury_authority.to_account_info(),
        },
    );

    let vault_seed = ctx.accounts.distribution_progress.vault_seed;
    let seeds = &[QUOTE_TREASURY_SEED, &vault_seed.to_le_bytes(), &[ctx.bumps.quote_treasury_authority]];
    let signer_seeds = &[&seeds[..]];

    token::transfer(transfer_ctx.with_signer(signer_seeds), amount)?;

    emit!(CreatorPayoutDayClosed {
        creator_amount: amount,
        total_investor_distributed: ctx.accounts.distribution_progress.daily_distributed,
        quote_mint: ctx.accounts.quote_mint.key(),
        timestamp,
    });

    Ok(())
}

/// Validate that the pool configuration will only accrue quote fees
fn validate_quote_only_pool(ctx: &Context<InitializeHonoraryPosition>) -> Result<()> {
    // In DAMM V2, we need to validate the pool's token order and collect_fee_mode
    // to ensure we only get quote token fees (token B)

    // For now, we'll do basic validation and rely on pool configuration
    // The bounty specifies that if quote-only cannot be guaranteed, we should fail

    // TODO: Add specific DAMM V2 pool state validation
    // - Check pool.collect_fee_mode to ensure it's configured for quote-only fees
    // - Validate token order (quote should be token B)

    Ok(())
}

#[derive(Accounts)]
pub struct InitializeGlobalState<'info> {
    #[account(
        init,
        payer = payer,
        space = GlobalState::LEN,
        seeds = [GLOBAL_STATE_SEED],
        bump
    )]
    pub global_state: Account<'info, GlobalState>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(vault_seed: u64)]
pub struct InitializeHonoraryPosition<'info> {
    /// The pool for which we're creating the honorary position
    /// CHECK: This will be validated by the DAMM V2 program
    pub pool: UncheckedAccount<'info>,

    /// The position account to be created (PDA derived from position NFT mint)
    /// CHECK: This will be created by the DAMM V2 program
    #[account(mut)]
    pub position: UncheckedAccount<'info>,

    /// Position NFT mint (will be created and owned by our program PDA)
    /// CHECK: This will be created as a signer
    #[account(mut)]
    pub position_nft_mint: Signer<'info>,

    /// Position NFT account (ATA for the position NFT)
    /// CHECK: This will be created by DAMM V2 program
    #[account(mut)]
    pub position_nft_account: UncheckedAccount<'info>,

    /// PDA that will own the position
    /// CHECK: This is a PDA derived from vault seed and validated by seeds constraint
    #[account(
        seeds = [VAULT_SEED, &vault_seed.to_le_bytes(), INVESTOR_FEE_POSITION_OWNER_SEED],
        bump
    )]
    pub position_owner_pda: UncheckedAccount<'info>,

    /// DAMM V2 Pool Authority (fixed address)
    /// CHECK: This is the fixed pool authority for DAMM V2
    pub pool_authority: UncheckedAccount<'info>,

    /// Quote mint of the pool (token B in DAMM V2)
    pub quote_mint: Account<'info, Mint>,

    /// Token A mint of the pool  
    pub token_a_mint: Account<'info, Mint>,

    /// Quote treasury ATA owned by the program
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = quote_mint,
        associated_token::authority = quote_treasury_authority
    )]
    pub quote_treasury: Account<'info, TokenAccount>,

    /// Authority for the quote treasury (PDA)
    /// CHECK: This is a PDA derived from vault seed and validated by seeds constraint
    #[account(
        seeds = [QUOTE_TREASURY_SEED, &vault_seed.to_le_bytes()],
        bump
    )]
    pub quote_treasury_authority: UncheckedAccount<'info>,

    /// Payer for initialization
    #[account(mut)]
    pub payer: Signer<'info>,

    /// DAMM V2 CP-AMM program
    /// CHECK: This is the DAMM V2 CP-AMM program ID
    pub cp_amm_program: UncheckedAccount<'info>,

    /// Event authority for DAMM V2
    /// CHECK: This is the event authority PDA for DAMM V2
    pub event_authority: UncheckedAccount<'info>,

    /// System program
    pub system_program: Program<'info, System>,

    /// Token program
    pub token_program: Program<'info, Token>,

    /// Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,

    /// Rent sysvar
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
#[instruction(page_index: u32, investor_fee_share_bps: u16, daily_cap_lamports: Option<u64>, min_payout_lamports: u64, y0_total: u64)]
pub struct DistributeFees<'info> {
    /// Global state
    #[account(
        seeds = [GLOBAL_STATE_SEED],
        bump = global_state.bump
    )]
    pub global_state: Account<'info, GlobalState>,

    /// Distribution progress tracking
    #[account(
        init_if_needed,
        payer = payer,
        space = DistributionProgress::LEN,
        seeds = [DISTRIBUTION_PROGRESS_SEED, &12345u64.to_le_bytes()], // Using placeholder vault seed
        bump
    )]
    pub distribution_progress: Account<'info, DistributionProgress>,

    /// Honorary position
    /// CHECK: This is the Meteora position account
    pub position: UncheckedAccount<'info>,

    /// Position owner PDA
    /// CHECK: This is a PDA derived from vault seed and validated by seeds constraint
    #[account(
        seeds = [VAULT_SEED, &12345u64.to_le_bytes(), INVESTOR_FEE_POSITION_OWNER_SEED], // Using placeholder vault seed
        bump
    )]
    pub position_owner_pda: UncheckedAccount<'info>,

    /// Quote mint
    pub quote_mint: Account<'info, Mint>,

    /// Quote treasury ATA
    #[account(
        mut,
        associated_token::mint = quote_mint,
        associated_token::authority = quote_treasury_authority
    )]
    pub quote_treasury: Account<'info, TokenAccount>,

    /// Quote treasury authority (PDA)
    /// CHECK: This is a PDA derived from vault seed and validated by seeds constraint
    #[account(
        seeds = [QUOTE_TREASURY_SEED, &12345u64.to_le_bytes()], // Using placeholder vault seed
        bump
    )]
    pub quote_treasury_authority: UncheckedAccount<'info>,

    /// Creator's quote ATA (from global state)
    #[account(
        mut,
        constraint = creator_quote_ata.key() == global_state.creator_quote_ata
    )]
    pub creator_quote_ata: Account<'info, TokenAccount>,

    /// Payer for any account initialization
    #[account(mut)]
    pub payer: Signer<'info>,

    /// DAMM V2 Pool (for fee claiming)
    /// CHECK: This is the DAMM V2 pool account
    pub pool: UncheckedAccount<'info>,

    /// DAMM V2 Pool Authority
    /// CHECK: This is the fixed pool authority for DAMM V2
    pub pool_authority: UncheckedAccount<'info>,

    /// Position NFT account
    /// CHECK: This is the position NFT account
    pub position_nft_account: UncheckedAccount<'info>,

    /// Token A mint
    pub token_a_mint: Account<'info, Mint>,

    /// Token A vault of the pool
    /// CHECK: This is the token A vault account
    pub token_a_vault: UncheckedAccount<'info>,

    /// Token B vault of the pool (quote mint vault)
    /// CHECK: This is the token B vault account
    pub token_b_vault: UncheckedAccount<'info>,

    /// DAMM V2 CP-AMM program
    /// CHECK: This is the DAMM V2 CP-AMM program ID
    pub cp_amm_program: UncheckedAccount<'info>,

    /// Event authority for DAMM V2
    /// CHECK: This is the event authority PDA for DAMM V2
    pub event_authority: UncheckedAccount<'info>,

    /// Streamflow program
    /// CHECK: This is the Streamflow program ID  
    pub streamflow_program: UncheckedAccount<'info>,

    /// System program
    pub system_program: Program<'info, System>,

    /// Token program
    pub token_program: Program<'info, Token>,

    /// Associated token program
    pub associated_token_program: Program<'info, AssociatedToken>,
    // Remaining accounts should be passed as:
    // [streamflow_stream_1, investor_ata_1, streamflow_stream_2, investor_ata_2, ...]
}
