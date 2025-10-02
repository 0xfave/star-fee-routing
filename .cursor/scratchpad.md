# Star Fee Routing - Meteora DLMM V2 Honorary Position Project

## Background and Motivation

This project implements a Superteam bounty to build a permissionless fee routing Anchor program for Meteora DLMM V2. The program needs to:

1. Create and manage an "honorary" DLMM v2 LP position owned by a program PDA that accrues fees in quote mint only
2. Provide a permissionless 24-hour crank mechanism that distributes quote fees to investors pro-rata based on still-locked amounts from Streamflow, with remainder going to creator

**Bounty Prize:** 7,500 USDC
**Deadline:** October 17, 2025 (8 days remaining)
**Key Requirements:** Quote-only fees, 24h distribution crank, Streamflow integration, pro-rata distribution

## Key Challenges and Analysis

### Current Implementation Status
The project has a partially working Anchor program with:
- ✅ Basic program structure and PDA seeds defined
- ✅ Global state management for creator quote ATA
- ✅ Events defined (HonoraryPositionInitialized, QuoteFeesClaimed, InvestorPayoutPage, CreatorPayoutDayClosed)
- ✅ Error handling with custom error codes
- ✅ State structures (GlobalState, DistributionProgress, PolicyConfig, InvestorData)
- ⚠️ Partial honorary position initialization (missing actual Meteora integration)
- ⚠️ Placeholder distribution logic (missing Streamflow integration)
- ❌ Tests are not working (test failure with exit code 101)
- ❌ Missing actual Meteora DLMM V2 program integration
- ❌ Missing Streamflow program integration for locked token queries
- ❌ Missing quote-only validation logic
- ❌ Missing pagination support for multiple investors

### Technical Challenges Identified
1. **Meteora DLMM V2 Integration**: Need to understand the exact CPI calls for creating positions and claiming fees
2. **Quote-Only Validation**: Must ensure position configuration only accrues quote fees, failing cleanly if base fees would occur
3. **Streamflow Integration**: Need to query locked amounts from Streamflow streams for pro-rata calculation
4. **Pagination**: Support for distributing to multiple investors across multiple transaction calls
5. **Idempotent Operations**: Ensure crank can be safely re-run without double payments

## High-level Task Breakdown

### Work Package A - Initialize Honorary Fee Position (Quote-Only)
- [ ] **A1**: Research and integrate actual Meteora DLMM V2 program calls for position creation
- [ ] **A2**: Implement quote-only validation logic based on pool configuration
- [ ] **A3**: Create position owned by program PDA with correct seeds
- [ ] **A4**: Add proper account validation for pool token order
- [ ] **A5**: Test position creation with quote-only accrual

### Work Package B - Permissionless 24h Distribution Crank
- [ ] **B1**: Integrate Streamflow program calls to query locked amounts
- [ ] **B2**: Implement proper fee claiming from honorary position via Meteora CPI
- [ ] **B3**: Add pagination support for multiple investors
- [ ] **B4**: Implement pro-rata distribution calculation with floor math
- [ ] **B5**: Add daily cap and dust threshold handling
- [ ] **B6**: Ensure idempotent operations and resumable pagination
- [ ] **B7**: Complete creator remainder distribution

### Work Package C - Testing and Documentation
- [ ] **C1**: Fix existing test failures and create comprehensive test suite
- [ ] **C2**: Add end-to-end tests with simulated Meteora positions and Streamflow
- [ ] **C3**: Test edge cases (all unlocked, base fee detection, pagination)
- [ ] **C4**: Create comprehensive README with integration steps
- [ ] **C5**: Document all account requirements and PDA seeds

### Work Package D - Final Integration and Validation
- [ ] **D1**: Validate against all bounty acceptance criteria
- [ ] **D2**: Performance testing and optimization
- [ ] **D3**: Final code review and cleanup
- [ ] **D4**: Prepare deliverables for submission

## Project Status Board

### Current Sprint
- [ ] Fix current test failures to establish baseline
- [ ] Research Meteora DLMM V2 program documentation and CPI interface
- [ ] Research Streamflow program interface for locked token queries
- [ ] Set up proper test environment with mocked external programs

### Completed Tasks
- [x] Basic Anchor program structure created
- [x] State accounts and PDAs defined
- [x] Events and error handling implemented
- [x] Basic distribution logic skeleton
- [x] DAMM V2 integration started - position creation CPI call implemented
- [x] DAMM V2 fee claiming CPI call implemented
- [x] Updated account structures for DAMM V2 compatibility

### In Progress
- [x] Updated program to use DAMM V2 (cp-amm) instead of DLMM
- [x] Added DAMM V2 IDL file
- [x] Implemented basic DAMM V2 CPI calls for position creation and fee claiming
- [ ] Testing DAMM V2 integration (current focus)

### Blocked/Need Help
- [ ] Need Meteora DLMM V2 program documentation and examples
- [ ] Need Streamflow program interface documentation
- [ ] Need to understand exact quote-only validation requirements

## Current Status / Progress Tracking

**Last Updated:** October 2, 2025

**Current Phase:** DAMM V2 Integration and Implementation
**Overall Progress:** ~35% complete

The program now successfully builds with DAMM V2 integration. Basic CPI calls for position creation and fee claiming are implemented. The next major milestone is completing the Streamflow integration and implementing proper quote-only validation.

**Immediate Priorities:**
1. Complete Streamflow integration for locked token queries
2. Implement proper quote-only validation for DAMM V2 pools
3. Add pagination support for multiple investors
4. Create comprehensive test suite with mocked external programs

## Executor's Feedback or Assistance Requests

### Questions for Human User
1. Should I proceed as Executor to fix the failing tests first, or would you prefer Planner mode to further refine the approach?
2. Do you have access to Meteora DLMM V2 program documentation or example code?
3. Do you have Streamflow program interface documentation?
4. Should I use devnet/testnet program IDs for testing or create mock programs?

### Technical Decisions Needed
- Which approach for quote-only validation: runtime validation vs preflight simulation?
- How to handle missing investor ATAs during distribution?
- Pagination strategy: fixed page size or dynamic based on transaction size limits?

## Lessons

### Development Guidelines
- Include info useful for debugging in the program output
- Read the file before trying to edit it
- If there are vulnerabilities in terminal, run npm audit before proceeding
- Always ask before using -force git command

### Project-Specific Learnings
- The current test setup is basic and needs proper integration test framework
- Placeholder vault seeds (12345) are used throughout - these need to be parameterized
- The program structure follows Anchor best practices and now has basic DAMM V2 integration
- DAMM V2 uses different concepts than DLMM (no ticks, uses concentrated liquidity positions)
- CPI calls to external programs require careful account ordering and signer seed management
- DAMM V2 program ID: cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG
- Pool authority is fixed: HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC
