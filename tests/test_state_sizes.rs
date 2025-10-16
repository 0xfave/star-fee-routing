// Test for state account sizes
use star_fee_routing::state::{DistributionProgress, GlobalState, PolicyConfig};

mod common;

#[test]
fn test_state_sizes() {
    println!("🧪 Testing State Account Sizes");

    // Test GlobalState size
    let global_state_size = GlobalState::LEN;
    println!("GlobalState size: {} bytes", global_state_size);
    assert_eq!(global_state_size, 8 + 32 + 1); // discriminator + pubkey + bump

    // Test DistributionProgress size
    let progress_size = DistributionProgress::LEN;
    println!("DistributionProgress size: {} bytes", progress_size);
    assert_eq!(progress_size, 8 + 8 + 8 + 8 + 4 + 1 + 8 + 1); // all fields

    // Test PolicyConfig size
    let policy_size = PolicyConfig::LEN;
    println!("PolicyConfig size: {} bytes", policy_size);
    assert_eq!(policy_size, 8 + 2 + 9 + 8 + 8 + 8 + 1);

    println!("✅ All state sizes validated");
}
