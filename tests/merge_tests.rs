use hyperloglog::HyperLogLog;



#[test]
fn merge_with_same_precision_test() {

    let p = 10;

    let mut hll: HyperLogLog<i32> = HyperLogLog::new(p);
    
    for i in 0..10 {
        hll.insert(i);
    }

    let mut hll2: HyperLogLog<i32> = HyperLogLog::new(p);

    for i in 10..20 {
        hll2.insert(i);
    }

    let res = hll.merge(&hll2);

    assert!(res.is_ok(), "Error while merging");
}

#[test]
fn merge_with_different_precisions_test() {
    let p1 = 10;
    let p2 = 20;

    let mut hll: HyperLogLog<i32> = HyperLogLog::new(p1);

    let mut hll2: HyperLogLog<i32> = HyperLogLog::new(p2);

    for i in 1..20 {
        hll.insert(i);
        hll2.insert(i + 10);
    }

    let result = hll.merge(&hll2);

    assert!(result.is_err(), "Able to merge with different precision")

}

#[test]
fn merge_with_empty_does_nothing_test() {
    let p = 10;

    let mut hll: HyperLogLog<i32> = HyperLogLog::new(p);
    let mut hll2: HyperLogLog<i32> = HyperLogLog::new(p);

    for i in 1..10_000 {
        hll.insert(i);
    }

    let original = hll.calculate_cardinality(); 

    let result = hll.merge(&hll2);

    assert!(result.is_ok(), "Error while merging with same precision");

    assert_eq!(
        hll.calculate_cardinality(),
        original,
        "Merging an empty HLL should not change the estimate"
    );
}

