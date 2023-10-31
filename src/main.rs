use std::net::{ SocketAddrV4, Ipv4Addr };
use std::io::{Write, Read};
use btc_handshake::{version::*, network::MAGIC_NUMBER, btc_message::{BtcMessage, ToTheChain}};


fn handshake() {
    
    let address = SocketAddrV4::new(Ipv4Addr::new(127,0,0,1),18445);
    let mut stream = std::net::TcpStream::connect(address).unwrap();

    let version_msg = VersionMessage::new(60002i32, address);
    let payload = version_msg.to_rawmessage().unwrap();
    let vers_check : [u8; 4] = version_msg.calculate_sha256()[..4].try_into().unwrap();
    let checksum = u32::from_ne_bytes(vers_check);

    let btc_m = BtcMessage::new(MAGIC_NUMBER, VersionMessage::COMMAND, checksum, payload);
    // Send version message.
    stream.write_all(&btc_m.to_bytes()).unwrap();
    stream.flush().unwrap();

    let mut rec_buff = [0;24];
    // Receive version payload.
    println!("Waiting for verack answer...");

    let _ = stream.read_exact(&mut rec_buff);  // return the size of the buffer (don't need it now)

    let v_answer = BtcMessage::from_bytes(&mut rec_buff).unwrap();
    if v_answer.command() != VersionMessage::COMMAND {
        println!("{:?}", v_answer.command());
        println!("{:?}", VersionMessage::COMMAND);
        println!("ERROR: Wrong command");
    } else {
        println!("connection established. Msg received {:?}", v_answer.command());
    }
}


// const BITCOIN_PROTOCOL_VERSION: i32 = 70016; // matches bitcoin core v24
fn main() {
    handshake();
}
