// Common test utilities and helpers
use anchor_lang::prelude::Pubkey as AnchorPubkey;
use litesvm::LiteSVM;
use solana_sdk::{
    native_token::LAMPORTS_PER_SOL,
    pubkey::Pubkey,
    signature::{Keypair, Signer},
};
use std::path::PathBuf;

// Convert Anchor Pubkey to Solana Pubkey
pub fn anchor_to_solana_pubkey(anchor_pk: &AnchorPubkey) -> Pubkey {
    Pubkey::from(anchor_pk.to_bytes())
}

// Convert Solana Pubkey to Anchor Pubkey
pub fn solana_to_anchor_pubkey(solana_pk: &Pubkey) -> AnchorPubkey {
    AnchorPubkey::from(solana_pk.to_bytes())
}

/// Setup function that initializes LiteSVM and loads the program
pub fn setup() -> (LiteSVM, Keypair) {
    // Initialize LiteSVM and payer
    let mut svm = LiteSVM::new();
    let payer = Keypair::new();

    // Airdrop some SOL to the payer keypair
    svm.airdrop(&payer.pubkey(), 10 * LAMPORTS_PER_SOL).expect("Failed to airdrop SOL to payer");

    // Load program SO file
    let so_path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join("target/deploy/star_fee_routing.so");

    let program_data =
        std::fs::read(&so_path).unwrap_or_else(|_| panic!("Failed to read program SO file at {:?}", so_path));

    let program_id = anchor_to_solana_pubkey(&star_fee_routing::ID);
    svm.add_program(program_id.to_bytes(), &program_data).expect("Failed to add program");

    println!("✅ LiteSVM setup complete");
    println!("Program ID: {}", program_id);
    println!("Payer: {}", payer.pubkey());

    // Return the LiteSVM instance and payer keypair
    (svm, payer)
}
