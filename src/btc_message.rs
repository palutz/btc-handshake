use std::io::Error;
use crate::utils::{parse_frombytes_le, read_drop_slice};

pub trait ToTheChain {
    fn to_bytes(&self) -> Vec<u8>;
    fn from_bytes(bytes: &[u8]) -> Result<Box<Self>, Error>;
}

#[derive(Debug, Default)]
// Defining the format of the message going to the chain
pub struct BtcMessage {
    magic: u32,
    command: [u8; 12],
    length: u32,
    checksum: u32,
    payload: Vec<u8>,
}

impl BtcMessage {
    pub fn new(magic: u32, command: [u8; 12], checksum: u32, payload: Vec<u8>) -> Self {
        Self {
            magic,
            command,
            length: payload.len() as u32,
            checksum,
            payload,
        }
    }

    pub fn command(&self) -> [u8;12] {
        //std::str::from_utf8(&self.command).unwrap()
        self.command
    }
}

/// Message structure (see https://en.bitcoin.it/wiki/Protocol_documentation#Message_structure)
///
/// size | field    | type     | description
/// ---  | -----    | ----     | ------------
/// 4    | magic    | u32      | Magic value 
/// 12   | command  | [u8; 12] | ASCII string i
/// 4    | length   | u32      | Length of payload in number of bytes
/// 4    | checksum | u32      | First 4 bytes of sha256(sha256(payload))
/// ?    | payload  | Vec<u8>  | The actual data
impl ToTheChain for BtcMessage {
    fn to_bytes(&self) -> Vec<u8> {
        let mut buff = Vec::new();
        buff.extend_from_slice(&self.magic.to_le_bytes());
        buff.extend_from_slice(&self.command);
        buff.extend_from_slice(&self.length.to_le_bytes());
        buff.extend_from_slice(&self.checksum.to_ne_bytes());
        buff.extend_from_slice(&self.payload);
        buff
    }

    fn from_bytes(bytes: &[u8]) -> Result<Box<Self>, Error> {
        println!("{:?}",bytes);
        let (magic, buff) = parse_frombytes_le::<u32>(&bytes.to_vec())?;
        let (cmd, buff) = read_drop_slice(&buff, 12)?;
        println!("{:?}", cmd);
        let command = <[u8; 12]>::try_from(cmd).unwrap();
        let (length, buff) = parse_frombytes_le::<u32>(&buff)?;
        let (checksum, _) = parse_frombytes_le::<u32>(&buff)?;

        Ok(Box::new(Self {
            magic,
            command,
            length,
            checksum,
            payload: Vec::new(), // not checking the payload 
        }))
    }
}