use std::time::UNIX_EPOCH;
use rand::{Rng, thread_rng};
use core::ops::BitAnd;

pub fn calculate_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}


pub fn generate_random_nonce() -> u64 {
    let mut rng = thread_rng(); // random gen
    rng.gen::<u64>()
}

pub fn nodenetwork_bitmask(node_net: &i32) -> u64 {
    let mut buffer : u64 = 0x0;
    buffer = buffer.bitand(*node_net as u64);
    buffer
}


// Generic parser using the FromBytes trait from the num (num_traits) crate
// could be a trait and maybe better to put a constrait on the type
pub fn parse_frombytes_be<'a, T>(buff: &Vec<u8>) -> Result<(T, Vec<u8>), std::io::Error> 
where T : FromEndian + Sized
{
    let size = core::mem::size_of::<T>(); 
    match read_drop_slice(buff, size) {
        Ok((res, remaining)) => Ok((FromEndian::from_be(&res), remaining)),
        Err(e) => Err(e),
    }
}

pub fn parse_frombytes_le<'a, T>(buff: &Vec<u8>) -> Result<(T, Vec<u8>), std::io::Error> 
where T : FromEndian + Sized
{
    let size = core::mem::size_of::<T>(); 
    match read_drop_slice(buff, size) {
        Ok((res, remaining)) => Ok((FromEndian::from_le(&res), remaining)),
        Err(e) => Err(e),
    }
}
// if correct, return the parsed value and the new vector without it , similar to Parsec
// pub fn read_drop_slice<'a>(buff: &Vec<u8>, size: usize) -> Result<(&'a [u8], Vec<u8>), std::io::Error> {
pub fn read_drop_slice(buff: &Vec<u8>, size: usize) -> Result<(&[u8], Vec<u8>), std::io::Error> {
    if buff.len() >= size {
        Ok((&buff[0..size], buff[size..].to_vec()))
    } else {
        Err(std::io::Error::new(std::io::ErrorKind::UnexpectedEof, "Buffer too small"))
    }
}

// An attempt of a generic BigEndian/LittleEndian parser for numeric types
pub trait FromEndian {
    fn from_be(msg: &[u8]) -> Self
    where
        Self : Sized;
    fn from_le(msg: &[u8]) -> Self
    where
        Self : Sized;
}

impl FromEndian for i32 {
    fn from_be(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(msg);
        i32::from_be_bytes(bytes)
    }
    fn from_le(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(msg);
        i32::from_le_bytes(bytes)
    }
}

impl FromEndian for i64 {
    fn from_be(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(msg);
        i64::from_be_bytes(bytes)
    }
    fn from_le(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(msg);
        i64::from_le_bytes(bytes)
    }
}

impl FromEndian for u16 {
    fn from_be(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(msg);
        u16::from_be_bytes(bytes)
    }
    fn from_le(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 2];
        bytes.copy_from_slice(msg);
        u16::from_le_bytes(bytes)
    }
}

impl FromEndian for u32 {
    fn from_be(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(msg);
        u32::from_be_bytes(bytes)
    }
    fn from_le(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 4];
        bytes.copy_from_slice(msg);
        u32::from_le_bytes(bytes)
    }
}

impl FromEndian for u64 {
    fn from_be(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(msg);
        u64::from_be_bytes(bytes)
    }
    fn from_le(msg: &[u8]) -> Self {
        let mut bytes = [0u8; 8];
        bytes.copy_from_slice(msg);
        u64::from_le_bytes(bytes)
    }
}

