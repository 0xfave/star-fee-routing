#!/bin/bash
# Script to pull programs from mainnet for testing

set -e

echo "📦 Pulling programs from Solana Mainnet for testing..."

# Create fixtures directory if it doesn't exist
mkdir -p fixtures

# DAMM V2 (CP-AMM) Program
CP_AMM_PROGRAM_ID="cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG"
echo "Pulling DAMM V2 (CP-AMM) program: $CP_AMM_PROGRAM_ID"
if [ ! -f "fixtures/cp_amm.so" ]; then
    solana program dump $CP_AMM_PROGRAM_ID fixtures/cp_amm.so --url mainnet-beta
    echo "✅ Downloaded CP-AMM program"
else
    echo "⏭️  CP-AMM program already exists"
fi

# Streamflow Program  
# The Streamflow program ID is: strmRqUCoQUgGUan5YhzUZa6KqdzwX5L6FpUxfmKg5m
STREAMFLOW_PROGRAM_ID="strmRqUCoQUgGUan5YhzUZa6KqdzwX5L6FpUxfmKg5m"
echo "Pulling Streamflow program: $STREAMFLOW_PROGRAM_ID"
if [ ! -f "fixtures/streamflow.so" ]; then
    solana program dump $STREAMFLOW_PROGRAM_ID fixtures/streamflow.so --url mainnet-beta
    echo "✅ Downloaded Streamflow program"
else
    echo "⏭️  Streamflow program already exists"
fi

echo ""
echo "✅ All programs downloaded to fixtures/"
echo ""
echo "Program files:"
ls -lh fixtures/*.so
