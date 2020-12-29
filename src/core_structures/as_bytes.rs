pub trait AsBytes {
    fn as_kv_bytes(&self) -> Vec<u8>;
}

impl AsBytes for String {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.as_bytes().to_vec()
    }
}

impl AsBytes for u8 {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl AsBytes for u16 {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl AsBytes for u32 {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl AsBytes for u64 {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl AsBytes for u128 {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl AsBytes for usize {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl AsBytes for i8 {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl AsBytes for i16 {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl AsBytes for i32 {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl AsBytes for i64 {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl AsBytes for i128 {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl AsBytes for isize {
    fn as_kv_bytes(&self) -> Vec<u8> {
        self.to_le_bytes().to_vec()
    }
}

impl<T: AsBytes> AsBytes for Vec<T> {
    fn as_kv_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        for entry in self {
            bytes.append(&mut entry.as_kv_bytes());
        }

        bytes
    }
}
