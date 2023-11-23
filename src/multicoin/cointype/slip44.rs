use ethers_core::types::U256;

use super::CoinType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SLIP44 {
    Hedera,
    Ripple,
    Solana,
    Cardano,
    Stellar,
    Bitcoin,
    Binance,
    Litecoin,
    Dogecoin,
    Ethereum,
    Polkadot,
    Rootstock,
    EthereumClassic,
    Other(U256),
}

impl From<u32> for SLIP44 {
    fn from(val: u32) -> SLIP44 {
        match val {
            0 => SLIP44::Bitcoin,
            2 => SLIP44::Litecoin,
            3 => SLIP44::Dogecoin,
            60 => SLIP44::Ethereum,
            61 => SLIP44::EthereumClassic,
            144 => SLIP44::Ripple,
            148 => SLIP44::Stellar,
            3030 => SLIP44::Hedera,
            1815 => SLIP44::Cardano,
            137 => SLIP44::Rootstock,
            714 => SLIP44::Binance,
            501 => SLIP44::Solana,
            354 => SLIP44::Polkadot,
            val => SLIP44::Other(val.into()),
        }
    }
}

impl From<SLIP44> for CoinType {
    fn from(val: SLIP44) -> Self {
        CoinType::Slip44(val)
    }
}
