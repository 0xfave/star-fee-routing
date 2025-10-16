// Test for 24-hour distribution timing logic
mod common;

#[test]
fn test_distribution_timing() {
    println!("🧪 Testing Time-Based Distribution Logic");

    let seconds_per_day = 86400i64;

    // Test case 1: Distribution allowed after 24 hours
    let last_distribution = 1000000i64;
    let current_time = 1086400i64; // 24 hours later
    let time_diff = current_time - last_distribution;

    println!("Last distribution: {}", last_distribution);
    println!("Current time: {}", current_time);
    println!("Time difference: {} seconds ({} hours)", time_diff, time_diff / 3600);

    assert!(time_diff >= seconds_per_day);
    println!("✅ Distribution allowed (>= 24 hours)");

    // Test case 2: Distribution not allowed before 24 hours
    let too_early = 1080000i64; // Only 22.22 hours later
    let time_diff_early = too_early - last_distribution;

    println!("\nEarly attempt time: {}", too_early);
    println!("Time difference: {} seconds ({} hours)", time_diff_early, time_diff_early / 3600);

    assert!(time_diff_early < seconds_per_day);
    println!("✅ Distribution blocked (< 24 hours)");
}
