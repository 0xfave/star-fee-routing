// Test for fee calculation logic
mod common;

#[test]
fn test_fee_calculation_logic() {
    println!("🧪 Testing Fee Calculation Logic");

    // Test scenario: 1000 tokens total fee, 80% to investors
    let total_fees = 1_000_000_000u64;
    let investor_share_bps = 8000u32;

    let investor_amount =
        (total_fees as u128).checked_mul(investor_share_bps as u128).unwrap().checked_div(10000u128).unwrap() as u64;

    let creator_amount = total_fees - investor_amount;

    println!("Total fees: {}", total_fees);
    println!("Investor share (80%): {}", investor_amount);
    println!("Creator share (20%): {}", creator_amount);

    assert_eq!(investor_amount, 800_000_000);
    assert_eq!(creator_amount, 200_000_000);

    // Test pro-rata distribution
    let total_locked = 5_000_000u64;
    let investor1_locked = 2_000_000u64;
    let investor2_locked = 3_000_000u64;

    let investor1_share = (investor1_locked as u128)
        .checked_mul(investor_amount as u128)
        .unwrap()
        .checked_div(total_locked as u128)
        .unwrap() as u64;

    let investor2_share = (investor2_locked as u128)
        .checked_mul(investor_amount as u128)
        .unwrap()
        .checked_div(total_locked as u128)
        .unwrap() as u64;

    println!("Investor 1 share (40% locked): {}", investor1_share);
    println!("Investor 2 share (60% locked): {}", investor2_share);

    assert_eq!(investor1_share, 320_000_000);
    assert_eq!(investor2_share, 480_000_000);

    println!("✅ Fee calculation logic validated");
}
