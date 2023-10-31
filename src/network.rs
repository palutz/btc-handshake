
pub const MAGIC_NUMBER: u32 = 0xDAB5BFFA;  //Regtest/testnet

#[derive(Debug)]
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