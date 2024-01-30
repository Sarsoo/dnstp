
const BYTEMASK_32: u32 = 0b11111111;
const BYTEMASK_16: u16 = 0b11111111;

pub fn two_byte_extraction(buffer: &[u8], idx: usize) -> u16
{
    ((buffer[idx] as u16) << 8) | buffer[idx + 1] as u16
}

pub fn two_byte_split(num: u16) -> (u8, u8)
{
    ((num >> 8) as u8,
     (num & BYTEMASK_16) as u8)
}

pub fn four_byte_split(num: u32) -> (u8, u8, u8, u8)
{
    ((num >> 24) as u8,
     ((num >> 16) & BYTEMASK_32) as u8,
     ((num >> 8) & BYTEMASK_32) as u8,
     (num & BYTEMASK_32) as u8)
}

pub fn apply_split_bytes(buffer: &mut [u8], value: u16, index: usize)
{
    let val = two_byte_split(value);
    buffer[index] = val.0;
    buffer[index + 1] = val.1;
}
