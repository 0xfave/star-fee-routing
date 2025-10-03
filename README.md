# Star Fee Routing - Meteora DLMM V2 Integration

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)](https://www.rust-lang.org)
[![Anchor](https://img.shields.io/badge/anchor-0.31.1-blue.svg)](https://anchor-lang.com)

> ⚠️ **CRITICAL DISCLAIMER**: This code is **NOT AUDITED** and is provided for educational/development purposes only. Using this code in production environments or with real funds carries significant financial risk. The author(s) assume **NO RESPONSIBILITY** for any financial losses, security vulnerabilities, or other damages that may result from using this code. **USE AT YOUR OWN RISK**.

A permissionless fee routing Anchor program for Meteora DLMM V2 that manages honorary quote-only fee positions and distributes fees to investors based on their locked token amounts from Streamflow contracts.

## 🎯 Overview

This program implements the Superteam bounty specification for a permissionless fee routing system with two core components:

### **Work Package A**: Honorary Quote-Only Fee Position
- Creates DLMM V2 positions owned by program PDAs that exclusively accrue quote-mint fees
- Validates pool configuration to guarantee no base-token fees are collected
- Implements deterministic failure if quote-only collection cannot be ensured

### **Work Package B**: 24-Hour Distribution Crank
- Permissionless crank callable once per 24-hour window with pagination support
- Claims fees from honorary positions via CPI to Meteora's `cp-amm` program
- Queries real-time locked token amounts from Streamflow contracts
- Distributes fees pro-rata based on investor lock percentages
- Routes remainder to creator after investor distributions

## 🏗️ Architecture

### Core Components

```
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  DLMM V2 Pool   │───▶│  Honorary        │───▶│  Quote Treasury │
│  (Meteora)      │    │  Fee Position    │    │  (Program PDA)  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
                                │                        │
                                ▼                        ▼
┌─────────────────┐    ┌──────────────────┐    ┌─────────────────┐
│  Streamflow     │───▶│  Distribution    │───▶│  Investor ATAs  │
│  Contracts      │    │  Crank (24h)     │    │  + Creator ATA  │
└─────────────────┘    └──────────────────┘    └─────────────────┘
```

### Mathematical Model

The program implements precise mathematical formulas for fee distribution:

- **Locked Ratio**: `f_locked(t) = locked_total(t) / Y0` ∈ [0, 1]
- **Eligible Share**: `eligible_investor_share_bps = min(investor_fee_share_bps, floor(f_locked(t) * 10000))`
- **Investor Allocation**: `investor_fee_quote = floor(claimed_quote * eligible_investor_share_bps / 10000)`
- **Pro-rata Distribution**: `weight_i(t) = locked_i(t) / locked_total(t)`, payout: `floor(investor_fee_quote * weight_i(t))`

## 🔧 Installation & Setup

### Prerequisites

- **Rust**: 1.70.0 or higher
- **Solana CLI**: 1.16.0 or higher  
- **Anchor CLI**: 0.31.1 or higher
- **Node.js**: 18.0.0 or higher (for tests)

### Build Instructions

```bash
# Clone the repository
git clone https://github.com/0xfave/star-fee-routing.git
cd star-fee-routing

# Install dependencies
yarn install

# Build the program
anchor build

# Run tests (optional)
anchor test
```

### Program Deployment

```bash
# Configure Solana for desired network
solana config set --url https://api.devnet.solana.com  # or mainnet-beta

# Deploy the program
anchor deploy

# Verify deployment
solana program show <PROGRAM_ID>
```

## 📋 Integration Guide

### Step 1: Initialize Global State

```typescript
const globalStateInit = await program.methods
  .initializeGlobalState(creatorQuoteAta)
  .accounts({
    globalState: globalStatePda,
    payer: wallet.publicKey,
    systemProgram: SystemProgram.programId,
  })
  .rpc();
```

### Step 2: Create Honorary Position

```typescript
const honoraryPosInit = await program.methods
  .initializeHonoraryPosition(vaultSeed)
  .accounts({
    pool: dlmmPoolPubkey,
    position: positionPda,
    positionOwnerPda: positionOwnerPda,
    quoteMint: quoteMintPubkey,
    // ... additional DLMM V2 accounts
  })
  .rpc();
```

### Step 3: Run Distribution Crank

```typescript
const distributeFees = await program.methods
  .distributeFees(
    tradeAmount,
    feePercentage,
    pageIndex,
    investorFeeShareBps,
    dailyCapLamports,
    minPayoutLamports,
    y0Total
  )
  .accounts({
    globalState: globalStatePda,
    distributionProgress: progressPda,
    position: honoraryPositionPda,
    quoteGrunt: quoteTreasuryPda,
    creatorQuoteAta: creatorAta,
    // ... additional accounts
  })
  .remainingAccounts([
    // Pairs of: [streamflowContract1, investorAta1, streamflowContract2, investorAta2, ...]
  ])
  .rpc();
```

## 📊 Account Structure

### Program Derived Addresses (PDAs)

| Account Type | Seeds | Purpose |
|--------------|-------|---------|
| **Global State** | `["global_state"]` | Stores creator configuration |
| **Position Owner** | `["vault", vault_seed, "investor_fee_pos_owner"]` | Controls honorary position |
| **Quote Treasury** | `["quote_treasury", vault_seed]` | Holds claimed fees |
| **Distribution Progress** | `["distribution_progress", vault_seed]` | Tracks daily distribution state |

### State Accounts

#### GlobalState
```rust
pub struct GlobalState {
    pub creator_quote_ata: Pubkey,  // Creator's quote token destination
    pub bump: u8,                   // PDA bump seed
}
```

#### DistributionProgress  
```rust
pub struct DistributionProgress {
    pub last_distribution_ts: i64,  // Unix timestamp of last distribution
    pub daily_distributed: u64,     // Total distributed today
    pub carry_over: u64,            // Dust carried from previous pages
    pub page_cursor: u32,           // Current pagination cursor
    pub day_complete: bool,         // Whether day's distribution is finished
    pub vault_seed: u64,            // Associated vault identifier
    pub bump: u8,                   // PDA bump seed
}
```

## 🎛️ Configuration Parameters

### Required Inputs

| Parameter | Type | Description |
|-----------|------|-------------|
| `vault_seed` | `u64` | Unique identifier for position derivation |
| `investor_fee_share_bps` | `u32` | Basis points allocated to investors (0-10000) |
| `daily_cap_lamports` | `Option<u64>` | Optional daily distribution limit |
| `min_payout_lamports` | `u64` | Minimum payout threshold (dust prevention) |
| `y0_total` | `u64` | Total investor allocation at Token Generation Event |

### Policy Examples

```rust
// Conservative: 60% to investors, 2 SOL daily cap, 0.01 SOL minimum
investor_fee_share_bps: 6000,
daily_cap_lamports: Some(2_000_000_000),
min_payout_lamports: 10_000_000,

// Aggressive: 90% to investors, no cap, 0.001 SOL minimum  
investor_fee_share_bps: 9000,
daily_cap_lamports: None,
min_payout_lamports: 1_000_000,
```

## 🔒 Security Features

### Quote-Only Enforcement
- **Pool Validation**: Verifies token order ensures quote-mint is token B
- **Runtime Checks**: Validates no base-token fees during claiming
- **Deterministic Failure**: Aborts distribution if base fees detected

### Access Control
- **PDA Ownership**: All positions owned by program-derived addresses
- **Permissionless Cranks**: Anyone can call distribution (prevents censorship)
- **Time Gating**: 24-hour minimum between distribution cycles

### Financial Protections
- **Daily Caps**: Optional limits prevent excessive distributions
- **Dust Handling**: Carries forward small amounts to prevent waste
- **Minimum Thresholds**: Prevents uneconomical micro-transactions

## 📅 Operational Workflow

### Daily Distribution Cycle

1. **Initialization** (Day 0):
   - Deploy program and initialize global state
   - Create honorary position in target DLMM pool
   - Configure distribution parameters

2. **Fee Accrual** (Days 1-N):
   - Honorary position automatically collects quote-only fees
   - Fees accumulate in DLMM position until next distribution

3. **Distribution Crank** (Every 24+ hours):
   - **Page 0**: Claims fees from position, queries Streamflow locks
   - **Page 1-N**: Distributes to investor batches (pagination)
   - **Final Page**: Routes remainder to creator, marks day complete

4. **Monitoring**:
   - Track `DistributionProgress` account for daily status
   - Monitor events for successful distributions and errors
   - Verify fee flows to investor and creator accounts

## 📡 Events & Monitoring

### Emitted Events

```rust
// Position creation
HonoraryPositionInitialized {
    position: Pubkey,
    position_owner: Pubkey,
    vault_seed: u64,
    quote_mint: Pubkey,
}

// Fee claiming
QuoteFeesClaimed {
    amount_claimed: u64,
    quote_mint: Pubkey,  
    timestamp: i64,
}

// Investor distributions
InvestorPayoutPage {
    page_index: u32,
    investor_count: u32,
    total_distributed: u64,
    timestamp: i64,
}

// Creator remainder
CreatorPayoutDayClosed {
    creator_amount: u64,
    total_investor_distributed: u64,
    quote_mint: Pubkey,
    timestamp: i64,
}
```

## ⚠️ Error Codes

| Code | Error | Description |
|------|-------|-------------|
| `6000` | `BaseFeeDetected` | Pool would result in base token fees |
| `6001` | `TooEarlyForDistribution` | 24-hour window not elapsed |
| `6002` | `InvalidQuoteMint` | Quote mint validation failed |
| `6003` | `NoFeesAvailable` | No fees to claim from position |
| `6004` | `BaseFeesClaimedError` | Base fees detected during claim |
| `6005` | `ArithmeticOverflow` | Mathematical operation overflow |
| `6006` | `InvalidPageIndex` | Pagination cursor mismatch |
| `6007` | `DistributionAlreadyComplete` | Day already completed |
| `6008` | `InsufficientLockedTokens` | No locked tokens found |
| `6009` | `DailyCapExceeded` | Distribution exceeds daily limit |
| `6010` | `PayoutBelowThreshold` | Amount below minimum payout |
| `6011` | `InvalidStreamflowContract` | Streamflow data deserialization failed |

## 🧪 Testing

### Test Coverage

The program includes comprehensive tests covering:

- **Unit Tests**: Individual function validation
- **Integration Tests**: End-to-end workflows with mock accounts
- **Edge Cases**: Empty distributions, caps exceeded, dust handling
- **Error Cases**: Invalid configurations, failed validations

### Running Tests

```bash
# Run all tests
anchor test

# Run specific test file
anchor test -- --test test_initialize

# Run with detailed output
anchor test -- --nocapture
```

### Test Scenarios

1. **Happy Path**: Full distribution cycle with multiple investors
2. **Edge Cases**: Single investor, zero fees, minimum thresholds
3. **Error Conditions**: Invalid pools, base fees, timing violations
4. **Pagination**: Large investor sets across multiple pages
5. **Streamflow Integration**: Various lock states and contract conditions

## 🔄 Dependencies

### Core Dependencies

```toml
[dependencies]
anchor-lang = { version = "0.31.1", features = ["init-if-needed"] }
anchor-spl = "0.31.1"
streamflow-sdk = { version = "0.10", features = ["cpi"] }
```

### External Program Dependencies

- **Meteora DLMM V2**: `CPMDWBwJDtYax9qW7AyRuVC19Cc4L4Vcy4n2BHAbHkCW`
- **Streamflow**: Program for token streaming/vesting contracts
- **SPL Token**: Standard Solana token operations

## 🚀 Production Checklist

Before deploying to mainnet:

- [ ] **Security Audit**: Engage professional auditors
- [ ] **Testnet Validation**: Extensive testing on devnet/testnet
- [ ] **Parameter Validation**: Verify all configuration values
- [ ] **Emergency Procedures**: Document pause/recovery mechanisms
- [ ] **Monitoring Setup**: Deploy tracking and alerting systems
- [ ] **Documentation Review**: Ensure all integration guides are accurate
- [ ] **Backup Plans**: Document manual intervention procedures

## 📜 Legal & Risk Disclaimers

### ⚠️ **IMPORTANT DISCLAIMERS**

1. **NO AUDIT**: This code has **NOT** been audited by security professionals
2. **FINANCIAL RISK**: Using this code may result in **TOTAL LOSS OF FUNDS**
3. **NO WARRANTY**: Code provided "AS IS" without any guarantees
4. **USER RESPONSIBILITY**: You assume **ALL RISKS** of using this software
5. **NO LIABILITY**: Authors accept **NO RESPONSIBILITY** for any losses or damages

### Risk Factors

- **Smart Contract Bugs**: Code may contain exploitable vulnerabilities
- **Integration Risks**: Dependencies on external programs (Meteora, Streamflow)
- **Economic Attacks**: MEV, sandwich attacks, or other DeFi exploits
- **Operational Errors**: Misconfiguration leading to fund loss
- **Regulatory Changes**: Legal/compliance risks in various jurisdictions

## 🤝 Contributing

### Development Guidelines

1. **Fork** the repository
2. **Create** feature branch: `git checkout -b feature/amazing-feature`
3. **Test** thoroughly: `anchor test`
4. **Document** changes in README and code comments
5. **Submit** pull request with detailed description

### Code Standards

- Follow Rust naming conventions and formatting
- Add comprehensive inline documentation
- Include test coverage for new features
- Maintain compatibility with Anchor 0.31.1+

## 📞 Support & Contact

- **Issues**: [GitHub Issues](https://github.com/0xfave/star-fee-routing/issues)
- **Discussions**: [GitHub Discussions](https://github.com/0xfave/star-fee-routing/discussions)
- **Superteam Bounty**: [Original Specification](https://earn.superteam.fun/listing/build-permissionless-fee-routing-anchor-program-for-meteora-dlmm-v2)

## 📄 License

This project is licensed under the MIT License - see the [LICENSE.md](LICENSE.md) file for details.

---

**⚠️ FINAL WARNING**: This software is experimental and unaudited. **DO NOT USE WITH REAL FUNDS** without proper security review and testing. The authors disclaim all liability for any financial losses.
