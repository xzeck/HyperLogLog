pub mod tobytes;
mod tolebytes;

pub use tobytes::ToBytes;         // re-export the trait

use xxhash_rust::xxh3::xxh3_64;

/// HyperLogLog is a probabilistic data structure for estimating cardinality.
/// This implementation uses the HyperLogLog algorithm to estimate the
/// number of distinct elements in a large stream of data, using `p`
/// bits (which determines the number of buckets).
pub struct HyperLogLog<T: ToBytes> {
    p: u32,
    m: usize,
    buckets: Vec<u64>,
    _marker: std::marker::PhantomData<T>,
}

impl<T: ToBytes> HyperLogLog<T> {
    /// Creates a new `HyperLogLog` with `p` bits.
    pub fn new(p: u32) -> Self {
        let m = 2_usize.pow(p);
        let mut buckets = Vec::with_capacity(m);
        buckets.resize(m, 0); // Avoid allocation overhead by resizing in place

        HyperLogLog {
            p,
            m,
            buckets,
            _marker: std::marker::PhantomData,
        }
    }

    /// Efficient hash function using `XxHash64` for faster hashing.
    fn hash_input(item: T) -> u64 {
        xxh3_64(&item.to_bytes())
    }

    /// Optimized cardinality calculation without extra allocations.
    pub fn calculate_cardinality(&self) -> u64 {
        let sum: f64 = self.buckets.iter().map(|&v| 2f64.powi(-(v as i32))).sum();

        let zero_buckets = self.buckets.iter().filter(|&&v| v == 0).count();

        // this would mean that no insertions have been made yet
        // and the set is empty so we can return a 0
        if zero_buckets == self.m {
            return 0; // Early return for empty set
        }

        let z = 1.0 / sum;

        let alpha_m = match self.m {
            16 => 0.673,
            32 => 0.697,
            64 => 0.709,
            m if m >= 128 => 0.7213 / (1.0 + 1.079 / (m as f64)),
            _ => panic!("Unsupported bucket count"),
        };

        let estimate = alpha_m * (self.m as f64).powi(2) * z;

        // small range linear correction
        if zero_buckets > 0 {
            let linear_count = (self.m as f64) * (self.m as f64) / (zero_buckets as f64).ln();
            if linear_count <= (2.5 * self.m as f64) {
                return linear_count.round() as u64;
            }
        }
        

        estimate.round() as u64
    }

    /// Inserts an element into the HyperLogLog structure.
    pub fn insert(&mut self, item: T) {
        let hash = Self::hash_input(item);
        let bucket_index = (hash >> (64 - self.p)) as usize;
        let remaining = hash << self.p;
        let leading = (remaining.leading_zeros() + 1) as u64;
        self.buckets[bucket_index] = self.buckets[bucket_index].max(leading);
    }
}