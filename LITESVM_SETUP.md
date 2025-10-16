# LiteSVM Integration Guide

This document explains how LiteSVM has been integrated into the star-fee-routing project following the [LiteSVM Getting Started Guide](https://www.litesvm.com/docs/getting-started).

## What is LiteSVM?

LiteSVM is a fast, lightweight Solana Virtual Machine for testing Solana programs. It provides:
- ✅ Fast test execution (no validator needed)
- ✅ Direct program testing
- ✅ Simple setup and configuration
- ✅ Full SPL token support

## Setup Steps Completed

### 1. Dependencies Added

Added to `programs/star-fee-routing/Cargo.toml`:

```toml
[dev-dependencies]
litesvm = "0.8.1"
litesvm-token = "0.8.1"
solana-sdk = "3.0.0"
solana-account = "3.0.0"
solana-instruction = "3.0.0"
solana-keypair = "3.0.0"
solana-message = "3.0.0"
solana-native-token = "3.0.0"
solana-pubkey = "3.0.0"
solana-sdk-ids = "3.0.0"
solana-signer = "3.0.0"
solana-transaction = "3.0.0"
```

### 2. Test Module Created

Created `/programs/star-fee-routing/src/tests/mod.rs` with:

- Helper functions to convert between Anchor and Solana Pubkey types
- `setup()` function that initializes LiteSVM and loads the program
- Test suite following LiteSVM best practices

### 3. Program Module Updated

Updated `/programs/star-fee-routing/src/lib.rs` to include the tests module:

```rust
pub mod errors;
pub mod events;
pub mod state;
mod tests;  // Added this line
```

## Test Structure

### Helper Functions

```rust
// Convert Anchor Pubkey to Solana Pubkey
fn anchor_to_solana_pubkey(anchor_pk: &anchor_lang::prelude::Pubkey) -> Pubkey

// Convert Solana Pubkey to Anchor Pubkey  
fn solana_to_anchor_pubkey(solana_pk: &Pubkey) -> anchor_lang::prelude::Pubkey
```

### Setup Function

```rust
fn setup() -> (LiteSVM, Keypair)
```

This function:
1. Initializes LiteSVM
2. Creates and airdrops SOL to a payer keypair
3. Loads the compiled program from `target/deploy/star_fee_routing.so`
4. Returns the configured LiteSVM instance and payer

### Test Functions

1. **`test_initialize_global_state()`** - ✅ Fully implemented
   - Creates a quote token mint (USDC)
   - Creates creator's associated token account
   - Derives global state PDA
   - Calls initialize_global_state instruction
   - Verifies the account data

2. **`test_create_quote_position()`** - 🚧 Placeholder
   - Sets up test environment
   - Creates quote and base token mints
   - Ready for CP-AMM integration testing

3. **`test_distribution_flow()`** - 🚧 Placeholder
   - Ready for 24-hour crank logic testing
   - Ready for fee distribution testing

## Running Tests

```bash
# Run all tests
cd programs/star-fee-routing && cargo test -- --show-output

# Run specific test
cd programs/star-fee-routing && cargo test test_initialize_global_state -- --show-output

# From project root
cargo test-sbf
```

## Test Output Example

```
🧪 Testing Initialize Global State
✅ LiteSVM setup complete
Program ID: 45soP1GyzrULnWjAasDnp23T1yDZpkhPsQD6qQ98Ttdg
Payer: HnWpNH9vieGwSeArQDpDLEQB6fRv3Ucp1qzbp3iqtw6T
Quote Mint: ENwP3SQw22mBycyDdCwczYu7jYRWkpEkbhh1hkCEAxhJ
Creator Quote ATA: DJZdJTiBXnb3QevM4CBHFexfrSo9WCL9L1ekihMnT1v8
Global State PDA: BP9AbaQmiKgEWMTAW1NuFaL4HsbuNW4QpixfLZVkmcXD

✅ Initialize Global State transaction successful
CUs Consumed: 9967
Tx Signature: 432QNJHDmK4XvoF2abhMv8nP3ceg5LeMeATtnCc5dZ9Q...
✅ Global state data verified
```

## Key Differences from Traditional solana-program-test

1. **No async/await** - LiteSVM tests are synchronous
2. **Direct execution** - No validator simulation, instant feedback
3. **Simpler setup** - Just load the .so file
4. **Better DX** - Clear transaction results with compute units consumed

## Type Conversion Notes

Due to version differences between Anchor's Solana dependencies and LiteSVM's, we need conversion functions:

- Anchor uses `solana-program` v2.x types
- LiteSVM uses `solana-sdk` v3.x types

The helper functions handle these conversions seamlessly.

## Next Steps

To expand the test suite:

1. **Mock CP-AMM Integration**
   - Load CP-AMM program or create mock
   - Test `create_quote_position` instruction

2. **Distribution Testing**
   - Test 24-hour crank mechanism
   - Test fee distribution to investors
   - Test Streamflow integration

3. **Integration Tests**
   - End-to-end flow testing
   - Multiple investor scenarios
   - Edge case handling

## References

- [LiteSVM Documentation](https://www.litesvm.com/docs)
- [LiteSVM Getting Started](https://www.litesvm.com/docs/getting-started)
- [Example Repository](https://github.com/ASCorreia/escrow-litesvm)
- [LiteSVM GitHub](https://github.com/LiteSVM/litesvm)
