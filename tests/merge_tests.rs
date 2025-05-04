use hyperloglog::HyperLogLog;



#[test]
fn merge_test() {

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