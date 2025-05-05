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
    let hll2: HyperLogLog<i32> = HyperLogLog::new(p);

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

#[test]
fn merge_with_clone_idempotent() {
    let mut a = HyperLogLog::<u64>::new(10);
    for v in 0..2_000 {
        a.insert(v);
    }
    let first_est = a.calculate_cardinality();

    // Merge with a *clone* of `a` rather than `a` itself
    let other = a.clone();
    let res = a.merge(&other);
    assert!(res.is_ok(), "a.merge(&other) failed: {:?}", res);

    let second_est = a.calculate_cardinality();
    assert_eq!(
        first_est, second_est,
        "Merging an HLL with its clone should not change the estimate"
    );
}

#[test]
fn merge_is_associative() {
    let mut a = HyperLogLog::<u64>::new(10);
    let mut b = HyperLogLog::<u64>::new(10);
    let mut c = HyperLogLog::<u64>::new(10);

    for v in 0..1_000 { a.insert(v); }
    for v in 500..1_500 { b.insert(v); }
    for v in 1_000..2_000 { c.insert(v); }

    // (a ∪ b) ∪ c
    let mut ab = a.clone();
    let res_ab_b = ab.merge(&b);
    assert!(res_ab_b.is_ok(), "ab.merge(&b) failed: {:?}", res_ab_b);
    let res_ab_c = ab.merge(&c);
    assert!(res_ab_c.is_ok(), "ab.merge(&c) failed: {:?}", res_ab_c);
    let est1 = ab.calculate_cardinality();

    // a ∪ (b ∪ c)
    let mut bc = b.clone();
    let res_bc_c = bc.merge(&c);
    assert!(res_bc_c.is_ok(), "bc.merge(&c) failed: {:?}", res_bc_c);
    let mut abc = a.clone();
    let res_abc_bc = abc.merge(&bc);
    assert!(res_abc_bc.is_ok(), "abc.merge(&bc) failed: {:?}", res_abc_bc);
    let est2 = abc.calculate_cardinality();

    assert!(
        (est1 as i64 - est2 as i64).abs() <= 2,
        "Merge should be associative ({} vs {})",
        est1,
        est2
    );
}


#[test]
fn merge_is_commutative() {
    let mut a = HyperLogLog::<u64>::new(10);
    let mut b = HyperLogLog::<u64>::new(10);

    // give them overlapping but distinct sets
    for v in 0..1_500 {
        a.insert(v);
    }
    for v in 1_000..2_500 {
        b.insert(v);
    }

    // a ∪ b
    let mut ab = a.clone();
    let res_ab = ab.merge(&b);
    assert!(res_ab.is_ok(), "ab.merge(&b) failed: {:?}", res_ab);
    let est_ab = ab.calculate_cardinality();

    // b ∪ a
    let mut ba = b.clone();
    let res_ba = ba.merge(&a);
    assert!(res_ba.is_ok(), "ba.merge(&a) failed: {:?}", res_ba);
    let est_ba = ba.calculate_cardinality();

    // They should agree (within ±1 due to randomness)
    assert!(
        (est_ab as i64 - est_ba as i64).abs() <= 1,
        "Merge should be commutative ({} vs {})",
        est_ab,
        est_ba
    );
}

#[test]
fn merge_union_cardinality_within_tolerance() {
    let p = 12;
    let mut a = HyperLogLog::<u32>::new(p);
    let mut b = HyperLogLog::<u32>::new(p);
    let n1 = 500;
    let n2 = 800;
    let overlap = 200;

    for v in 0..n1 { a.insert(v as u32); }
    for v in (n1 - overlap)..(n1 - overlap + n2) {
        b.insert(v as u32);
    }

    let mut u = a.clone();
    let res = u.merge(&b);
    assert!(res.is_ok(), "u.merge(&b) failed: {:?}", res);
    let estimate = u.calculate_cardinality();

    let exact_union = (n1 + n2 - overlap) as u64;
    let tolerance = (exact_union as f64 * 0.05) as u64; // 5%

    // Note: This relies on p value, if p value is high then precision also will be high so for lower p values this test breaks
    // TODO: Take a look at this test later
    assert!(
        (estimate >= exact_union.saturating_sub(tolerance))
            && (estimate <= exact_union + tolerance),
        "Estimated {} not within 5% of exact {}",
        estimate,
        exact_union
    );
}

#[test]
fn merge_makes_cardinality_almost_same_as_original_test() {
    let p = 10;

    let mut hll: HyperLogLog<i32> = HyperLogLog::new(p);

    let mut hll2: HyperLogLog<i32> = HyperLogLog::new(p);

    for i in 1..10_000 {
        hll.insert(i);
        hll2.insert(i);
    }

    let hll1_cardinality = hll.calculate_cardinality();
    let hll2_cardinality = hll2.calculate_cardinality();

    let result = hll.merge(&hll2);

    let cardinality_after_merge = hll.calculate_cardinality();

    assert!(result.is_ok(), "Cannot merge");

    assert!((
        (cardinality_after_merge >= hll2_cardinality && cardinality_after_merge <= hll1_cardinality) || 
        (cardinality_after_merge <=hll2_cardinality && cardinality_after_merge >= hll1_cardinality)
    )
    , "Cardinality wrong: {}", cardinality_after_merge);
}