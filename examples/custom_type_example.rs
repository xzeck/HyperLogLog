// 4) Sketching a custom type
use hyperloglog::{HyperLogLog, ToBytes};

#[derive(Clone)]
struct Person {
    id:   u64,
    name: String,
}

impl ToBytes for Person {
    fn to_bytes(&self) -> Vec<u8> {
        let mut v = self.id.to_le_bytes().to_vec();
        v.extend(self.name.as_bytes());
        v
    }

    const TYPE_ID: &'static [u8] = b"Person";
}

fn main() {
    let mut hll = HyperLogLog::<Person>::new(9).unwrap(); // 512 buckets

    let people = [
        Person { id: 1, name: "Alice".into() },
        Person { id: 2, name: "Bob".into()   },
        Person { id: 1, name: "Alice".into() }, // duplicate
    ];

    for person in &people {
        hll.insert(person.clone());
    }

    // Expect ~2 distinct Person entries
    println!("Distinct people: {}", hll.calculate_cardinality());
}
