// Test for global state initialization logic
use solana_sdk::pubkey::Pubkey;
use star_fee_routing::GLOBAL_STATE_SEED;

mod common;

#[test]
fn test_global_state_pda() {
    println!("🧪 Testing Initialize Global State PDA Derivation");

    let program_id = common::anchor_to_solana_pubkey(&star_fee_routing::ID);

    // Derive the global state PDA
    let (global_state, bump) = Pubkey::find_program_address(&[GLOBAL_STATE_SEED], &program_id);

    println!("Program ID: {}", program_id);
    println!("Global State PDA: {}", global_state);
    println!("Bump: {}", bump);

    // Verify PDA is valid
    assert_ne!(global_state, Pubkey::default());
    assert!(bump > 0 && bump <= 255);

    // Verify we can recreate it deterministically
    let (global_state2, bump2) = Pubkey::find_program_address(&[GLOBAL_STATE_SEED], &program_id);
    assert_eq!(global_state, global_state2);
    assert_eq!(bump, bump2);

    println!("✅ Global state PDA derivation validated");
}
