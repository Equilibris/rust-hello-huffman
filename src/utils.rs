#[allow(dead_code)]
pub mod math {

    pub const fn num_bits<T>() -> usize {
        std::mem::size_of::<T>() * 8
    }

    pub fn log_2(x: u32) -> u32 {
        assert!(x > 0);
        num_bits::<u32>() as u32 - x.leading_zeros() - 1
    }

    pub fn partition_u32_into_u8_array(input: u32) -> [u8; 4] {
        input.to_be_bytes()
    }

    #[cfg(test)]
    mod tests {
        use super::partition_u32_into_u8_array;

        #[test]
        fn it_partitions() {
            let test_data: u32 = 0x12345678;

            let data = partition_u32_into_u8_array(test_data);

            println!("{:#x}", data[0]);
            println!("{:#x}", data[1]);
            println!("{:#x}", data[2]);
            println!("{:#x}", data[3]);

            assert_eq!(data[0], 0x12 as u8);
            assert_eq!(data[1], 0x34 as u8);
            assert_eq!(data[2], 0x56 as u8);
            assert_eq!(data[3], 0x78 as u8);
        }
    }
}

#[macro_export]
macro_rules! push_bits {
    ($vec:expr, $bit_cursor:expr, $byte_cursor:expr, $value:expr, $size:expr) => {{
        use crate::bit_byte_increase;

        for (index, val) in $value.to_be_bytes().iter().enumerate() {
            let val = *val;
            $vec[$byte_cursor + index * (val > 0) as usize] |= val;
        }

        bit_byte_increase!($bit_cursor, $byte_cursor, $size);
    }};
}

#[macro_export]
macro_rules! bit_byte_increase {
    ($bit_cursor:expr, $byte_cursor:expr, $size:expr) => {{
        $bit_cursor += $size;
        $byte_cursor += ($bit_cursor) as usize / 8;
        $bit_cursor %= 8;
    }};
}
