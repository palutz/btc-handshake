use std::io::Error;
use std::net::{SocketAddrV4, Ipv4Addr};
use sha2::{Sha256,Digest};

use crate::utils::*;

pub trait ProtocolMessage {
    fn to_bytes(&self) -> Vec<u8>;
    // fn from_bytes(bytes: &[u8]) -> Self;
}

pub trait CommandMessage {
    const COMMAND: [u8; 12];
}

pub trait RawMessage {
    fn to_rawmessage(&self) -> Result<Vec<u8>, Error>;
    fn from_rawmessage(msg: &Vec<u8>) -> Result<Box<Self>, Error>;
}

pub trait NetworkAddress {
    fn netaddr_as_bytes(node_bitmask: &u64, address: &SocketAddrV4) -> Vec<u8>;
    fn netaddr_from_bytes(buff: &mut Vec<u8>) -> Result<SocketAddrV4, Error>;
}

pub trait Checksum {
    fn calculate_sha256(&self) -> [u8; 32];
}

/// https://en.bitcoin.it/wiki/Protocol_documentation#version
///
/// size | field        | type     | description
/// ---  | -----        | ----     | ------------
/// 4    | version      | i32      | Identifies protocol version being used by the node
/// 8    | services     | u64      | bitfield of features to be enabled for this connection
/// 8    | timestamp    | i64      | standard UNIX timestamp in seconds
/// 26   | addr_recv    | net_addr | The network address of the node receiving this message
/// 26   | addr_from    | net_addr | Field can be ignored.
/// 8    | nonce        | u64      | Node random nonce
/// ?    | user_agent   | var_str  | User Agent (0x00 if string is 0 bytes long)
/// 4    | start_height | i32      | The last block received by the emitting node
/// 1    | relay        | bool     | Whether the remote peer should announce relayed transactions or not, see BIP 0037
/// *********************************************************
/// Almost all integers are encoded in little endian. Only IP or port number are encoded big endian.
/// *********************************************************

#[derive(Debug)]
pub struct VersionMessage {
    pub protocol_version: i32,
    pub service: u64,
    pub timestamp: i64,
    pub addr_recv: SocketAddrV4,
    pub addr_from: SocketAddrV4,
    pub nonce: u64,
    pub user_agent: String,
    pub start_height: i32,
}

impl VersionMessage {
    pub fn new(protocol_version: i32, addr_recv: SocketAddrV4) -> Self {
        let timestamp = calculate_timestamp();
        VersionMessage {
            protocol_version,
            service: 0x1,
            timestamp,
            addr_recv,
            addr_from : SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 8080),
            nonce : generate_random_nonce(),
            user_agent: "".to_string(),
            start_height: 1,
        }
    }

}

impl RawMessage for VersionMessage {
    fn from_rawmessage(msg: &Vec<u8>) -> Result<Box<Self>, Error> {
        let (protocol_version, buff) = parse_frombytes_le::<i32>(msg)?;
        let (service, buff) = parse_frombytes_le::<u64>(&buff)?;
        let (timestamp, buff) = parse_frombytes_le::<i64>(&buff)?;

        let address = Self::netaddr_from_bytes(&mut buff.to_vec())?;
        let add_from = Self::netaddr_from_bytes(&mut buff.to_vec())?; 
        let(nonce, _) = parse_frombytes_le::<u64>(&buff)?; 
        // dropping the remaining fields ...

        Ok(Box::new(
            VersionMessage {
                protocol_version,
                service,
                timestamp,
                addr_recv: address,
                addr_from : add_from,
                nonce,
                user_agent: "".to_string(), // TODO let user_agent = parser.read_var_string()?;
                start_height: 1, // TODO let start_height = parser.read_i32_le()?;
        }))
    }

    fn to_rawmessage(&self) -> Result<Vec<u8>, Error> {
        let svc_bitmask = nodenetwork_bitmask(&0x1);
        let mut address_bytes = Self::netaddr_as_bytes(&svc_bitmask, &self.addr_recv);

        let mut buffer: Vec<u8> = vec!();
        buffer.extend_from_slice(&self.protocol_version.to_le_bytes());
        buffer.extend_from_slice(&svc_bitmask.to_le_bytes());
        buffer.extend_from_slice(&self.timestamp.to_le_bytes());
        buffer.append(&mut address_bytes);
        buffer.extend_from_slice(&[0x0_u8; 26]);  // addr_from
        buffer.extend_from_slice(&self.nonce.to_le_bytes());
        buffer.extend_from_slice(&[0]);  // user agent
        buffer.extend_from_slice(&self.start_height.to_le_bytes());
        buffer.extend_from_slice(&[0]);

        Ok(buffer)
    }
}

impl NetworkAddress for VersionMessage  {
    // supporting only Ipv4 address here ...
    fn netaddr_as_bytes(node_bitmask: &u64, address: &SocketAddrV4) -> Vec<u8> {
        let mut buffer : Vec<u8> = Vec::new();
        buffer.extend_from_slice(&node_bitmask.to_le_bytes());
        let ip_addr_bytes = address.ip().to_ipv6_compatible().octets();

        buffer.extend_from_slice(&ip_addr_bytes);
        buffer.extend_from_slice(&address.port().to_be_bytes());

        buffer
    }

    fn netaddr_from_bytes(buff: &mut Vec<u8>) -> Result<SocketAddrV4, Error> {

        let (_, buff) = parse_frombytes_le::<u64>(&buff)?; // node service field
        let (ip_addr, buff) = read_drop_slice(&buff, 16)?;
        let (port_addr, _) = parse_frombytes_be::<u16>(&buff)?;
        let array_ip = <[u8;4]>::try_from(ip_addr).unwrap();
        Ok(SocketAddrV4::new(Ipv4Addr::from(array_ip), port_addr))
    }
}


impl ProtocolMessage for VersionMessage {
    fn to_bytes(&self) -> Vec<u8> {
        match self.to_rawmessage() {
            Ok(b) => b,
            Err(_) => vec!(), // TODO - implement a proper error management
        }
    }
}

impl CommandMessage for VersionMessage {
    const COMMAND: [u8; 12] = *b"version\0\0\0\0\0";
}

impl Checksum for VersionMessage {
    fn calculate_sha256(&self) -> [u8; 32] {
        let payload = self.to_bytes();

        // double hash...
        let h1 = Sha256::default()
                .chain_update(payload)
                .finalize();

        let h2 = Sha256::default()
                .chain_update(h1)
                .finalize();

        h2.into()
    }
}


/// _A "verack" packet shall be sent if the version packet was accepted._
#[derive(Debug)]
pub struct VerackMessage {}

impl VerackMessage {
    pub fn new() -> Self {
        VerackMessage { }
    }
    pub fn to_raw_message(self) -> Vec<u8> {
        vec![]
    }
}

impl CommandMessage for VerackMessage {
    const COMMAND: [u8; 12] = *b"verack\0\0\0\0\0\0";
} 

impl RawMessage for VerackMessage {
    fn from_rawmessage(_msg: &Vec<u8>) -> Result<Box<Self>, Error> {
        Ok(Box::new(VerackMessage {}))
    }

    fn to_rawmessage(&self) -> Result<Vec<u8>, Error> {
        Ok(vec![])
    }
}