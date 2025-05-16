use hyperloglog::HyperLogLog;

#[test]
fn test_reset_clears_buckets() {
    let mut hll = HyperLogLog::<u32>::new(10).unwrap();

    // Insert some elements
    hll.insert(1);
    hll.insert(2);


    let are_buckets_filled: bool = hll.get_buckets().iter().any(|x| *x > 0u8);

    // Ensure buckets are not in the default state before reset
    assert!(are_buckets_filled == true);

    // Call reset
    hll.reset();

    let are_buckets_filled: bool = hll.get_buckets().iter().any(|x| *x > 0);

    assert!(are_buckets_filled == false);

}

#[test]
fn test_reset_does_not_affect_other_fields() {
    let mut hll = HyperLogLog::<u32>::new(10).unwrap();;

    let original_p = hll.get_p();
    let original_m = hll.get_m();

    // Call reset
    hll.reset();

    let new_p = hll.get_p();
    let new_m = hll.get_m();

    assert!(original_p == new_p, "original p: {}, not equal to p after reset: {}", original_p, new_p);
    assert!(original_m == new_m, "original m: {}, not equal to m after reset: {}", original_m, new_m);
}

#[test]
fn test_reset_after_inserting_elements() {
    let mut hll = HyperLogLog::<u32>::new(10).unwrap();;

    // Insert elements into the HyperLogLog
    hll.insert(1);
    hll.insert(2);

    // Ensure the cardinality estimate is non-zero
    let cardinality_before_reset = hll.calculate_cardinality();
    assert!(cardinality_before_reset > 0);

    // Call reset
    hll.reset();

    // Ensure the cardinality estimate is zero after reset
    let cardinality_after_reset = hll.calculate_cardinality();
    assert_eq!(cardinality_after_reset, 0);
}

#[test]
fn test_reset_multiple_times() {
    let mut hll = HyperLogLog::<u32>::new(10).unwrap();;

    // Call reset multiple times
    hll.reset();
    hll.reset();
    hll.reset();

    // Ensure buckets are still zero after multiple resets
    assert_eq!(hll.get_buckets()[0], 0);
}