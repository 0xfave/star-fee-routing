// Test for complete distribution flow
mod common;

#[test]
fn test_complete_flow_logic() {
    println!("🧪 Testing Complete Distribution Flow Logic");

    // Simulate a complete distribution flow
    let total_fees_claimed = 10_000_000u64;
    let investor_fee_share_bps = 8000u32;
    let y0_total = 100_000_000u64;
    let current_locked = 60_000_000u64;

    println!("=== Step 1: Claim Fees ===");
    println!("Total fees claimed: {}", total_fees_claimed);

    println!("\n=== Step 2: Calculate f_locked ===");
    let f_locked =
        (current_locked as u128).checked_mul(10000u128).unwrap().checked_div(y0_total as u128).unwrap() as u64;
    println!("f_locked: {} bps ({}%)", f_locked, f_locked / 100);

    println!("\n=== Step 3: Determine Eligible Share ===");
    let eligible_share = std::cmp::min(investor_fee_share_bps as u64, f_locked);
    println!("Eligible investor share: {} bps", eligible_share);

    println!("\n=== Step 4: Calculate Distribution ===");
    let investor_total =
        (total_fees_claimed as u128).checked_mul(eligible_share as u128).unwrap().checked_div(10000u128).unwrap()
            as u64;
    let creator_total = total_fees_claimed - investor_total;

    println!("Investor pool: {}", investor_total);
    println!("Creator amount: {}", creator_total);

    println!("\n=== Step 5: Pro-Rata to Investors ===");
    let investor1_locked = 30_000_000u64;
    let investor2_locked = 30_000_000u64;

    let inv1_share = (investor1_locked as u128)
        .checked_mul(investor_total as u128)
        .unwrap()
        .checked_div(current_locked as u128)
        .unwrap() as u64;

    let inv2_share = (investor2_locked as u128)
        .checked_mul(investor_total as u128)
        .unwrap()
        .checked_div(current_locked as u128)
        .unwrap() as u64;

    println!("Investor 1 payout: {}", inv1_share);
    println!("Investor 2 payout: {}", inv2_share);

    // Verify totals
    assert_eq!(investor_total, 6_000_000); // 60% of fees
    assert_eq!(creator_total, 4_000_000); // 40% of fees
    assert_eq!(inv1_share + inv2_share, investor_total);

    println!("\n✅ Complete distribution flow validated");
}
