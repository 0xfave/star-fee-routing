# Testing Star Fee Routing on Local Validator

## 1. Setup Local Validator with DAMM V2

```bash
# Start Solana local validator with increased account limits
solana-test-validator \
  --account cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG ./fixtures/cp-amm-program.so \
  --account HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC ./fixtures/pool-authority.json \
  --reset \
  --quiet
```

## 2. Deploy Your Program

```bash
# Build and deploy
anchor build
anchor deploy --provider.cluster localnet

# Or use anchor test with validator
anchor test --skip-local-validator
```

## 3. Test Flow Requirements

### A. Initialize Global State
```bash
# Test initializing the global state
anchor run initialize-global-state
```

### B. Create DAMM V2 Pool (Mock or Real)
```typescript
// Create a quote-only DAMM V2 pool for testing
const pool = await createDammV2Pool({
  tokenA: USDC_MINT,  // Quote token
  tokenB: SOL_MINT,   // Base token  
  quotesOnly: true    // Critical: only quote fees
});
```

### C. Initialize Honorary Position
```bash
# Test creating honorary fee position
anchor run initialize-honorary-position
```

### D. Test Fee Distribution
```bash
# Test the 24-hour distribution crank
anchor run distribute-fees
```

## 4. Key Test Scenarios

### Scenario 1: Quote-Only Validation
- ✅ Position should only accrue quote token fees
- ❌ Should reject positions that accrue base token fees

### Scenario 2: Time-Based Distribution
- ✅ Should allow distribution after 24 hours
- ❌ Should reject distribution before 24 hours

### Scenario 3: Streamflow Integration
- ✅ Should query locked amounts correctly
- ✅ Should calculate investor shares based on locked tokens

### Scenario 4: Permissionless Access
- ✅ Any wallet should be able to call distribute_fees
- ✅ Multiple vaults should work independently

## 5. Testing Commands

```bash
# Run all tests
anchor test

# Run specific test file
anchor test -- --test test_distribute

# Test with verbose output
anchor test -- --nocapture

# Test on local validator
anchor test --skip-local-validator
```

## 6. Validation Checklist

- [ ] Program builds without errors
- [ ] All PDA derivations work correctly  
- [ ] DAMM V2 CPI calls execute successfully
- [ ] Quote-only validation works
- [ ] 24-hour time lock enforced
- [ ] Fee distribution calculates correctly
- [ ] Streamflow integration functional
- [ ] Multiple concurrent vaults supported
- [ ] Error handling works properly
- [ ] Gas costs are reasonable

## 7. Mock Data Setup

```typescript
// Mock Streamflow data for testing
const mockInvestors = [
  { pubkey: investor1, lockedAmount: 1000 * 1e6 }, // 1000 USDC
  { pubkey: investor2, lockedAmount: 2000 * 1e6 }, // 2000 USDC  
  { pubkey: investor3, lockedAmount: 500 * 1e6 },  // 500 USDC
];

// Expected fee distribution: 
// investor1: 28.57% (1000/3500)
// investor2: 57.14% (2000/3500)  
// investor3: 14.29% (500/3500)
```
