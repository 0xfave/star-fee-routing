// Test for PDA derivations
use solana_sdk::pubkey::Pubkey;
use star_fee_routing::{
    DISTRIBUTION_PROGRESS_SEED, GLOBAL_STATE_SEED, INVESTOR_FEE_POSITION_OWNER_SEED, QUOTE_TREASURY_SEED, VAULT_SEED,
};

mod common;

#[test]
fn test_pda_derivations() {
    println!("🧪 Testing PDA Derivations");

    let program_id = common::anchor_to_solana_pubkey(&star_fee_routing::ID);
    let vault_seed = 12345u64;

    // Test global state PDA
    let (global_state, global_bump) = Pubkey::find_program_address(&[GLOBAL_STATE_SEED], &program_id);
    println!("Global State PDA: {} (bump: {})", global_state, global_bump);
    assert_ne!(global_state, Pubkey::default());
    assert!(global_bump > 0);

    // Test position owner PDA
    let (position_owner, position_bump) = Pubkey::find_program_address(
        &[VAULT_SEED, &vault_seed.to_le_bytes(), INVESTOR_FEE_POSITION_OWNER_SEED],
        &program_id,
    );
    println!("Position Owner PDA: {} (bump: {})", position_owner, position_bump);
    assert_ne!(position_owner, Pubkey::default());
    assert!(position_bump > 0);

    // Test quote treasury authority PDA
    let (treasury_auth, treasury_bump) =
        Pubkey::find_program_address(&[QUOTE_TREASURY_SEED, &vault_seed.to_le_bytes()], &program_id);
    println!("Treasury Authority PDA: {} (bump: {})", treasury_auth, treasury_bump);
    assert_ne!(treasury_auth, Pubkey::default());
    assert!(treasury_bump > 0);

    // Test distribution progress PDA
    let (progress_pda, progress_bump) =
        Pubkey::find_program_address(&[DISTRIBUTION_PROGRESS_SEED, &vault_seed.to_le_bytes()], &program_id);
    println!("Distribution Progress PDA: {} (bump: {})", progress_pda, progress_bump);
    assert_ne!(progress_pda, Pubkey::default());
    assert!(progress_bump > 0);

    // Verify all PDAs are unique
    assert_ne!(global_state, position_owner);
    assert_ne!(global_state, treasury_auth);
    assert_ne!(position_owner, treasury_auth);
    assert_ne!(progress_pda, global_state);

    println!("✅ All PDA derivations successful and unique");
}
