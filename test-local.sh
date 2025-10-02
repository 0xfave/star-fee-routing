#!/bin/bash

# Star Fee Routing - Local Validator Testing Script
# This script sets up and runs comprehensive testing for the Superteam bounty

echo "🎯 Star Fee Routing - Bounty Testing Setup"
echo "================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Configuration
PROGRAM_ID="45soP1GyzrULnWjAasDnp23T1yDZpkhPsQD6qQ98Ttdg"
CP_AMM_PROGRAM_ID="cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG"
POOL_AUTHORITY="HLnpSz9h2S4hiLQ43rnSD9XkcUThA7B8hQMKmDaiTLcC"

print_step() {
    echo -e "${BLUE}[STEP]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
print_step "Checking prerequisites..."

if ! command -v anchor &> /dev/null; then
    print_error "Anchor CLI not found. Please install Anchor framework."
    exit 1
fi

if ! command -v solana &> /dev/null; then
    print_error "Solana CLI not found. Please install Solana CLI."
    exit 1
fi

print_success "Prerequisites check passed"

# Configure Solana for local testing
print_step "Configuring Solana for local testing..."
solana config set --url localhost
solana config set --keypair ~/.config/solana/id.json

# Check if local validator is running
print_step "Checking local validator status..."
if ! solana cluster-version &> /dev/null; then
    print_warning "Local validator not running. Starting validator..."
    
    # Start local validator with DAMM V2 program (if available)
    echo "Starting Solana test validator..."
    solana-test-validator \
        --reset \
        --quiet &
    
    # Wait for validator to start
    sleep 5
    
    # Check if validator started successfully
    if ! solana cluster-version &> /dev/null; then
        print_error "Failed to start local validator"
        exit 1
    fi
    
    print_success "Local validator started"
else
    print_success "Local validator is running"
fi

# Airdrop SOL for testing
print_step "Airdropping SOL for testing..."
solana airdrop 10 --commitment confirmed
print_success "Airdropped 10 SOL"

# Build the program
print_step "Building Star Fee Routing program..."
if anchor build; then
    print_success "Program built successfully"
else
    print_error "Failed to build program"
    exit 1
fi

# Deploy the program
print_step "Deploying program to local validator..."
if anchor deploy --provider.cluster=localnet; then
    print_success "Program deployed successfully"
    echo "Program ID: $PROGRAM_ID"
else
    print_error "Failed to deploy program"
    exit 1
fi

# Run comprehensive tests
print_step "Running bounty requirement tests..."
echo ""
echo "🧪 Test Categories:"
echo "  1. PDA derivations and account structure"
echo "  2. DAMM V2 integration constants"  
echo "  3. Quote-only fee position validation"
echo "  4. 24-hour distribution timing"
echo "  5. Streamflow integration simulation"
echo "  6. Permissionless access validation"
echo ""

if anchor test --skip-local-validator; then
    print_success "All tests passed! 🎉"
    echo ""
    echo "📊 Test Results Summary:"
    echo "✅ PDA derivations working"
    echo "✅ DAMM V2 constants valid"
    echo "✅ Quote-only validation ready"
    echo "✅ Distribution timing logic ready"
    echo "✅ Streamflow integration structure ready"
    echo "✅ Permissionless design validated"
    echo ""
    echo "🎯 Bounty Requirements Status:"
    echo "✅ Core program structure complete"
    echo "✅ DAMM V2 integration implemented"
    echo "⚠️  Full CPI integration needs DAMM V2 program deployment"
    echo "⚠️  Streamflow integration needs real CPI calls"
    echo "⚠️  End-to-end testing needs external program mocks"
else
    print_error "Some tests failed"
    exit 1
fi

# Show next steps
echo ""
echo "🚀 Next Steps for Full Bounty Completion:"
echo "1. Deploy DAMM V2 program to local validator for CPI testing"
echo "2. Implement Streamflow CPI calls and test integration"
echo "3. Create end-to-end test scenarios with real token transfers"
echo "4. Test quote-only validation with actual DAMM V2 pools"
echo "5. Validate 24-hour distribution crank with time manipulation"
echo "6. Test multiple concurrent vaults and investors"
echo ""

print_success "Local validator testing setup complete!"
echo "Use 'anchor test --skip-local-validator' to run tests again"
