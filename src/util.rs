#[derive(Eq, PartialEq, Debug)]
pub enum BitReaderError {
    NotEnoughData,
    TooManyBitsForType,
}

/// A bit reader which reads bits from left to right.
pub struct BitReader<'a> {
    data: &'a [u8],
    index: usize,
}

impl<'a> BitReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        BitReader { data, index: 0 }
    }

    fn read_bit(&mut self) -> Result<u32, BitReaderError> {
        if self.index == self.data.len() * 8 {
            return Err(BitReaderError::NotEnoughData);
        }

        let val = self.data[self.index / 8] & (1 << self.index % 8);
        self.index += 1;

        Ok(if val == 0 { 0 } else { 1 })
    }

    // signed

    pub fn read_i16(&mut self, num_bits: usize) -> Result<i16, BitReaderError> {
        if num_bits > 16 {
            return Err(BitReaderError::TooManyBitsForType);
        };
        self.read_i32(num_bits).map(|v| v as i16)
    }

    pub fn read_i32(&mut self, num_bits: usize) -> Result<i32, BitReaderError> {
        if num_bits > 32 {
            return Err(BitReaderError::TooManyBitsForType);
        };

        let value = self.read_u32(num_bits)?;

        return if (value & (1 << (num_bits - 1))) != 0 {
            Ok(value.wrapping_sub(1 << num_bits) as i32)
        } else {
            Ok(value as i32)
        };
    }

    // unsigned

    pub fn read_u8(&mut self, num_bits: usize) -> Result<u8, BitReaderError> {
        if num_bits > 8 {
            return Err(BitReaderError::TooManyBitsForType);
        };
        self.read_u32(num_bits).map(|v| v as u8)
    }

    pub fn read_u16(&mut self, num_bits: usize) -> Result<u16, BitReaderError> {
        if num_bits > 16 {
            return Err(BitReaderError::TooManyBitsForType);
        };
        self.read_u32(num_bits).map(|v| v as u16)
    }

    pub fn read_u32(&mut self, num_bits: usize) -> Result<u32, BitReaderError> {
        if num_bits > 32 {
            return Err(BitReaderError::TooManyBitsForType);
        };

        let mut value = 0;
        for i in 0..num_bits {
            value |= self.read_bit()? << i;
        }
        Ok(value)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_bitreader() {
        let mut rdr = BitReader::new(&[0x1a, 0x2b, 0x3c, 0x4d, 0x5e, 0x6f, 0x78, 0x90]);

        assert_eq!(rdr.read_bit(), Ok(0));
        assert_eq!(rdr.read_bit(), Ok(1));
        assert_eq!(rdr.read_bit(), Ok(0));
        assert_eq!(rdr.read_bit(), Ok(1));
        assert_eq!(rdr.read_u16(6), Ok(0x31));
        assert_eq!(rdr.read_i16(6), Ok(0x0A));
        assert_eq!(rdr.read_i16(4), Ok(-0x04));
        assert_eq!(rdr.read_u16(11), Ok(0x4D3));
        assert_eq!(rdr.read_bit(), Ok(0));
        assert_eq!(rdr.read_u32(32), Ok(0x90786F5E));

        assert_eq!(rdr.read_bit(), Err(BitReaderError::NotEnoughData));
        assert_eq!(rdr.read_i16(17), Err(BitReaderError::TooManyBitsForType));
        assert_eq!(rdr.read_i32(33), Err(BitReaderError::TooManyBitsForType));
        assert_eq!(rdr.read_u32(33), Err(BitReaderError::TooManyBitsForType));
    }

    use super::*;
}
