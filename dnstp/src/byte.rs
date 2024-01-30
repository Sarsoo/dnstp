//! Utility functions for operating on bytes

/// 8-bit mask for wiping out bits past a byte. Formatted as a 32-bit number
const BYTEMASK_32: u32 = 0b11111111;
/// 8-bit mask for wiping out bits past a byte. Formatted as a 16-bit number
const BYTEMASK_16: u16 = 0b11111111;

/// Take two sequential bytes starting from idx in buffer and return a concatenated 2 byte number
pub fn two_byte_extraction(buffer: &[u8], idx: usize) -> u16
{
    ((buffer[idx] as u16) << 8) | buffer[idx + 1] as u16
}

/// Take a 2 byte number and split it in to it's two 8 bit halves
pub fn two_byte_split(num: u16) -> (u8, u8)
{
    ((num >> 8) as u8,
     (num & BYTEMASK_16) as u8)
}

/// Split a 32 bit number into it's 8-bit quartered components
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
