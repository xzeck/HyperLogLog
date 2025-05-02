pub mod utils {

    pub fn calculate_bounds(n: u64, tolerance: f64) -> (u64, u64) {
        let expected = n;
        let lower_bound = (expected as f64 * (1.0 - tolerance)).round() as u64;
        let upper_bound = (expected as f64 * (1.0 + tolerance)).round() as u64;

        return (lower_bound, upper_bound)
    }

}