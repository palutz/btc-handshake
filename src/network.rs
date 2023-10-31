
pub const MAGIC_NUMBER: u32 = 0xDAB5BFFA;  //Regtest/testnet

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Chain {
    Regtest,
    Testnet3,
}

impl Chain {
    pub fn magic_value(&self) -> u32 {
        match self {
            Chain::Regtest => 0xDAB5BFFA,
            Chain::Testnet3 => 0x0709110B
        }
    }
}
// 
// #[derive(Copy, Clone, Debug, PartialEq)]
// pub enum NodeService {
//     NodeNetwork = 0x1, // focusing only on one type of node atm
// }
// 
// impl NodeService {
// // will need a proper implementation (from and to) if implementing more than one node service
//     pub fn nodenetwork_bitmask(&self) -> u64 {
//         let mut buffer : u64 = 0x0;
//         buffer.bitand(*self as u64);
//         buffer
//     }
// }