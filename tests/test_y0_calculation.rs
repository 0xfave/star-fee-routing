// Test for Y0 (initial allocation) calculation
mod common;

#[test]
fn test_y0_calculation() {
    println!("🧪 Testing Y0 (Initial Allocation) Calculation");

    // Y0 = Total initial investor allocation at TGE
    let y0_total = 100_000_000u64; // 100M tokens initial allocation

    // Current locked amount (after some vesting)
    let current_locked = 60_000_000u64; // 60M tokens still locked

    // Calculate f_locked = (current_locked / y0_total) * 10000
    let f_locked =
        (current_locked as u128).checked_mul(10000u128).unwrap().checked_div(y0_total as u128).unwrap() as u64;

    println!("Y0 total: {}", y0_total);
    println!("Current locked: {}", current_locked);
    println!("f_locked: {} bps ({}%)", f_locked, f_locked / 100);

    assert_eq!(f_locked, 6000); // 60%

    // Test fee share calculation based on f_locked
    let investor_fee_share_bps = 8000u32; // Max 80% to investors
    let eligible_share = std::cmp::min(investor_fee_share_bps as u64, f_locked);

    println!("Max investor share: {} bps", investor_fee_share_bps);
    println!("Eligible share: {} bps", eligible_share);

    assert_eq!(eligible_share, 6000); // Capped by f_locked
    println!("✅ Y0 calculation validated");
}
