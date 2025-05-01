
#[cfg(test)]
mod IntegerTests {
    use super::*;

    #[test]
    fn test_insert_and_cardinality() {
        let mut hll = HyperLogLog::<i64>::new(10);
        let n = 10000i64;
        
        let tolerance = 0.02;
        
        let expected = n as u64;
        
        let (lower_bound, upper_bound) = calculate_bounds(n as i32, tolerance);
        
        let tests: Vec<i64> = (1..=n).collect();

        for val in tests.iter() {
            hll.insert(*val);
        }

        let estimated = hll.calculate_cardinality();
        
        assert!(
            estimated >= lower_bound && estimated <= upper_bound,
            "Estimate {} not within 2% of expected {} (range: {} - {})",
            estimated,
            expected,
            lower_bound,
            upper_bound
        );
    }

    #[test]
    fn test_empty_cardinality() {
        let hll = HyperLogLog::<i64>::new(10);
        assert_eq!(hll.calculate_cardinality(), 0);
    }

    #[test]
    fn test_large_random_numbers() {
        let mut hll = HyperLogLog::new(10);

        let n = 1_000_000_000;
        let tolerance = 0.02;

        for _ in 0u64..n {
            let value: i64 = rand::random();
            hll.insert(value);
        }

        let (upper_bound, lower_bound) = calculate_bounds(n, tolerance);

        let estimated = hll.calculate_cardinality();
        
        assert!(estimated >= lower_bound && estimated <= upper_bound,
        "Estimate {} not within 2% of expected {} (range: {} - {})",
        estimated,
        expected,
        lower_bound,
        upper_bound
        )

    }
}
