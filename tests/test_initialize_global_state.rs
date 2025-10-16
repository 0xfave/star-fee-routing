use anchor_lang::{AccountDeserialize, InstructionData, ToAccountMetas as AnchorToAccountMetas};
use litesvm_token::{CreateAssociatedTokenAccount, CreateMint};
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    message::Message,
    pubkey::Pubkey,
    signature::Signer,
    system_program,
    transaction::Transaction,
};
use star_fee_routing::GLOBAL_STATE_SEED;

mod common;

#[test]
fn test_initialize_global_state() {
    println!("🧪 Testing Initialize Global State");

    // Setup the test environment
    let (mut svm, payer) = common::setup();

    let program_id = common::anchor_to_solana_pubkey(&star_fee_routing::ID);

    // Create a mint for the quote token (e.g., USDC)
    let quote_mint = CreateMint::new(&mut svm, &payer).decimals(6).authority(&payer.pubkey()).send().unwrap();
    println!("Quote Mint: {}", quote_mint);

    // Create creator's associated token account for receiving fees
    let creator_quote_ata =
        CreateAssociatedTokenAccount::new(&mut svm, &payer, &quote_mint).owner(&payer.pubkey()).send().unwrap();
    println!("Creator Quote ATA: {}", creator_quote_ata);

    // Derive the global state PDA
    let (global_state, _bump) = Pubkey::find_program_address(&[GLOBAL_STATE_SEED], &program_id);
    println!("Global State PDA: {}", global_state);

    // Convert to anchor types for instruction building
    let anchor_global_state = common::solana_to_anchor_pubkey(&global_state);
    let anchor_payer = common::solana_to_anchor_pubkey(&payer.pubkey());
    let anchor_system_program = common::solana_to_anchor_pubkey(&SYSTEM_PROGRAM_ID);
    let anchor_creator_ata = common::solana_to_anchor_pubkey(&creator_quote_ata);

    // Get account metas from Anchor
    let anchor_account_metas = star_fee_routing::accounts::InitializeGlobalState {
        global_state: anchor_global_state,
        payer: anchor_payer,
        system_program: anchor_system_program,
    }
    .to_account_metas(None);

    // Convert AccountMeta types from Anchor to Solana
    let account_metas: Vec<AccountMeta> = anchor_account_metas
        .iter()
        .map(|meta| AccountMeta {
            pubkey: common::anchor_to_solana_pubkey(&meta.pubkey),
            is_signer: meta.is_signer,
            is_writable: meta.is_writable,
        })
        .collect();

    // Create the initialize_global_state instruction
    let initialize_ix = Instruction {
        program_id,
        accounts: account_metas,
        data: star_fee_routing::instruction::InitializeGlobalState { creator_quote_ata: anchor_creator_ata }.data(),
    };

    // Create and send the transaction
    let message = Message::new(&[initialize_ix], Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new(&[&payer], message, recent_blockhash);

    // Send the transaction
    let tx = svm.send_transaction(transaction).unwrap();

    println!("\n✅ Initialize Global State transaction successful");
    println!("CUs Consumed: {}", tx.compute_units_consumed);
    println!("Tx Signature: {}", tx.signature);

    // Verify the global state account
    let global_state_account = svm.get_account(&global_state).unwrap();
    let global_state_data =
        star_fee_routing::state::GlobalState::try_deserialize(&mut global_state_account.data.as_ref()).unwrap();

    assert_eq!(global_state_data.creator_quote_ata, anchor_creator_ata);
    println!("✅ Global state data verified");
}
