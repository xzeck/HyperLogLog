# HyperLogLog

This is a rust implementation of [Google's HyperLogLog](https://static.googleusercontent.com/media/research.google.com/en//pubs/archive/40671.pdf) algorithm

Usage:

```rust

use hyperloglog::HyperLogLog;
use std::{collections::HashSet, time::Instant};

fn main() {
    let mut hll: HyperLogLog<i64> = HyperLogLog::new(4);



    for i in -1_000_000i64..1_000_000 {
        hll.insert(i);
    }

    // getting an estimate    
    let estimate = hll.calculate_cardinality();

    // serializing the data
    let serialized = serde_json::to_string(&hll).unwrap();
    // output: {"p":4,"m":16,"buckets":[17,17,20,17,19,18,21,18,19,18,18,17,17,19,17,17],"fingerprint":17010847314131961531}
    println!("{}", serialized);

    // rebuild the data
    let desserialized: HyperLogLog<i64> = serde_json::from_str(&serialized).unwrap();

    println!("{}, {}", hll.calculate_cardinality(), desserialized.calculate_cardinality());
}

```

More examples can be found in `examples/`
