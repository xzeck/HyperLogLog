pub trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;

    const TYPE_ID: &'static [u8];
}

// Integer types (Little Endian)
impl ToBytes for u8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self]
    }

    const TYPE_ID: &'static [u8] = b"u8";
}

impl ToBytes for u16 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    const TYPE_ID: &'static [u8] = b"u16";
}

impl ToBytes for u32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    const TYPE_ID: &'static [u8] = b"u32";
}

impl ToBytes for u64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    const TYPE_ID: &'static [u8] = b"u64";
}

impl ToBytes for u128 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    const TYPE_ID: &'static [u8] = b"u128";
}

impl ToBytes for i8 {
    fn to_bytes(&self) -> Vec<u8> {
        vec![*self as u8]
    }

    const TYPE_ID: &'static [u8] = b"i8";

}

impl ToBytes for i16 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    const TYPE_ID: &'static [u8] = b"i16";
}

impl ToBytes for i32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    const TYPE_ID: &'static [u8] = b"i32";
}

impl ToBytes for i64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    const TYPE_ID: &'static [u8] = b"i64";
}

impl ToBytes for i128 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }

    const TYPE_ID: &'static [u8] = b"i128";
}

// Floating point types (Native Endian)
impl ToBytes for f32 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }

    const TYPE_ID: &'static [u8] = b"f32";
}

impl ToBytes for f64 {
    fn to_bytes(&self) -> Vec<u8> {
        self.to_ne_bytes().to_vec()
    }

    const TYPE_ID: &'static [u8] = b"f64";
}


impl ToBytes for &str {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }

    const TYPE_ID: &'static [u8] = b"&str";
}

impl ToBytes for String {
    fn to_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
    
    const TYPE_ID: &'static [u8] = b"String";
}
