// Test for pagination logic in multi-page distributions
mod common;

#[test]
fn test_pagination_logic() {
    println!("🧪 Testing Pagination Logic");

    // Simulate distribution across multiple pages
    let total_investors = 50;
    let investors_per_page = 10;
    let total_pages = (total_investors + investors_per_page - 1) / investors_per_page;

    println!("Total investors: {}", total_investors);
    println!("Investors per page: {}", investors_per_page);
    println!("Total pages needed: {}", total_pages);

    assert_eq!(total_pages, 5);

    // Test page cursor progression
    for page_index in 0..total_pages {
        let start_idx = page_index * investors_per_page;
        let end_idx = std::cmp::min(start_idx + investors_per_page, total_investors);
        let investors_in_page = end_idx - start_idx;

        println!("Page {}: investors {}-{} ({} total)", page_index, start_idx, end_idx - 1, investors_in_page);

        if page_index < total_pages - 1 {
            assert_eq!(investors_in_page, investors_per_page);
        }
    }

    println!("✅ Pagination logic validated");
}
