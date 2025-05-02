mod utils;

use hyperloglog::HyperLogLog;
use crate::utils::utils::calculate_bounds;

#[cfg(test)]
mod string_test {

    use super::*;

    #[test]
    fn test_string() {
        let mut hll = HyperLogLog::<String>::new(10);

        let n = 10000;

        let tolerance = 0.02;
        
        let expected = n as u64;

        let (lower_bound, upper_bound) = calculate_bounds(n, tolerance);


        let tests: Vec<String> = (1..n)
                                        .map(|i| 
                                            format!("test{}", i))
                                            .collect();

        for val in tests.iter() {
            hll.insert(val.to_string());
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
}