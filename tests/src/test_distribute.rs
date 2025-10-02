use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use star_fee_routing::{
    DISTRIBUTION_PROGRESS_SEED, GLOBAL_STATE_SEED, INVESTOR_FEE_POSITION_OWNER_SEED, QUOTE_TREASURY_SEED, VAULT_SEED,
};

const CP_AMM_PROGRAM_ID: &str = "cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG";
const POOL_AUTHORITY: &str = "HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC";

#[tokio::test]
async fn test_damm_v2_constants() {
    // Test that DAMM V2 program constants are valid
    let cp_amm_program = Pubkey::from_str(CP_AMM_PROGRAM_ID).unwrap();
    let pool_authority = Pubkey::from_str(POOL_AUTHORITY).unwrap();

    println!("✅ DAMM V2 Constants Test");
    println!("📍 CP-AMM Program: {}", cp_amm_program);
    println!("📍 Pool Authority: {}", pool_authority);

    // Verify these are valid pubkeys
    assert_ne!(cp_amm_program, Pubkey::default());
    assert_ne!(pool_authority, Pubkey::default());
    assert_ne!(cp_amm_program, pool_authority);

    println!("✅ All DAMM V2 constants are valid pubkeys");
}

#[tokio::test]
async fn test_pda_derivations() {
    // Test that all PDA derivations work correctly
    let program_id = Pubkey::from_str("45soP1GyzrULnWjAasDnp23T1yDZpkhPsQD6qQ98Ttdg").unwrap();
    let vault_seed = 12345u64;

    let (global_state_pda, global_state_bump) = Pubkey::find_program_address(&[GLOBAL_STATE_SEED], &program_id);

    let (position_owner_pda, position_owner_bump) = Pubkey::find_program_address(
        &[VAULT_SEED, &vault_seed.to_le_bytes(), INVESTOR_FEE_POSITION_OWNER_SEED],
        &program_id,
    );

    let (quote_treasury_authority, treasury_authority_bump) =
        Pubkey::find_program_address(&[QUOTE_TREASURY_SEED, &vault_seed.to_le_bytes()], &program_id);

    let (distribution_progress_pda, distribution_progress_bump) =
        Pubkey::find_program_address(&[DISTRIBUTION_PROGRESS_SEED, &vault_seed.to_le_bytes()], &program_id);

    println!("✅ PDA Derivation Tests");
    println!("📍 Global State: {} (bump: {})", global_state_pda, global_state_bump);
    println!("📍 Position Owner: {} (bump: {})", position_owner_pda, position_owner_bump);
    println!("📍 Treasury Authority: {} (bump: {})", quote_treasury_authority, treasury_authority_bump);
    println!("📍 Distribution Progress: {} (bump: {})", distribution_progress_pda, distribution_progress_bump);

    // Verify bumps are valid (should be > 0)
    assert!(global_state_bump > 0);
    assert!(position_owner_bump > 0);
    assert!(treasury_authority_bump > 0);
    assert!(distribution_progress_bump > 0);

    // Verify PDAs are different
    assert_ne!(global_state_pda, position_owner_pda);
    assert_ne!(global_state_pda, quote_treasury_authority);
    assert_ne!(position_owner_pda, quote_treasury_authority);

    println!("✅ All PDA derivations are valid and unique");
}
