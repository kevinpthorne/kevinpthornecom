pub struct Bitset {
    data: Vec<u8>,
}

impl Bitset {
    pub fn new(size: usize) -> Self {
        let bytes_needed = (size + 7) / 8;
        Self { data: vec![0; bytes_needed] }
    }

    pub fn from_u32(value: u32) -> Self {
        let mut bitset = Self::new(32); // Allocate for 32 bits (size of u32)
        for i in (0..32).rev() {
            bitset.set(i, (value >> i) & 1 == 1);
        }
        bitset
    }

    pub fn set(&mut self, index: usize, value: bool) {
        if index >= self.size() {
            panic!("Index out of bounds");
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.data[byte_index] = (self.data[byte_index] & !(1 << bit_index)) | ((value as u8) << bit_index);
    }

    pub fn clear(&mut self, index: usize) {
        if index >= self.size() {
            panic!("Index out of bounds");
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        self.data[byte_index] &= !(1 << bit_index);
    }

    pub fn get(&self, index: usize) -> bool {
        if index >= self.size() {
            panic!("Index out of bounds");
        }
        let byte_index = index / 8;
        let bit_index = index % 8;
        (self.data[byte_index] & (1 << bit_index)) != 0
    }

    fn size(&self) -> usize {
        self.data.len() * 8
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitset() {
        let mut bitset = Bitset::new(10);
        bitset.set(2, true);
        bitset.set(7, true);
        assert_eq!(bitset.get(2), true);
        assert_eq!(bitset.get(7), true);
        assert_eq!(bitset.get(1), false);
        bitset.clear(7);
        assert_eq!(bitset.get(7), false);
    }

    #[test]
    fn test_from_u32() {
        let value: u32 = 0b1011;
        let bitset = Bitset::from_u32(value);
        assert_eq!(bitset.get(0), true); // Least Sig Bit
        assert_eq!(bitset.get(1), true);
        assert_eq!(bitset.get(2), false);
        assert_eq!(bitset.get(3), true); // Most Sig Bit
    }
}
