use cerium::display::layout::width::Width;

#[test]
fn test_cache_hit() {
    let mut calc = Width::new();

    // First measurement - cache miss
    let width1 = calc.measure_text_cached("test");
    assert_eq!(calc.cache_size(), 1);

    // Second measurement - cache hit
    let width2 = calc.measure_text_cached("test");
    assert_eq!(width1, width2);
    assert_eq!(calc.cache_size(), 1); // No new entry
}

#[test]
fn test_clear_cache() {
    let mut calc = Width::new();
    calc.measure_text_cached("test");
    assert_eq!(calc.cache_size(), 1);

    calc.clear_cache();
    assert_eq!(calc.cache_size(), 0);
}
