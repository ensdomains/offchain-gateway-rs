use thiserror::Error;

use super::cointype::{coins::CoinType, slip44::SLIP44};

use self::checksum_address::EvmEncoder;

// use crate::multicoin::encoding::binance::BinanceDecoder;
// use crate::multicoin::encoding::bitcoin_cash::BitcoinCashDecoder;
// use crate::multicoin::encoding::cardano::CardanoDecoder;
// use crate::multicoin::encoding::dogecoin::DogecoinDecoder;
// use crate::multicoin::encoding::hedera::HederaDecoder;
// use crate::multicoin::encoding::litecoin::LitecoinDecoder;
// use crate::multicoin::encoding::monacoin::MonacoinDecoder;
// use crate::multicoin::encoding::monero::MoneroDecoder;
// use crate::multicoin::encoding::polkadot::PolkadotDecoder;
// use crate::multicoin::encoding::ripple::RippleDecoder;
// use crate::multicoin::encoding::solana::SolanaDecoder;
// use crate::multicoin::encoding::stellar::StellarDecoder;
// use crate::multicoin::encoding::tezos::TezosDecoder;

// pub mod binance;
// pub mod bitcoin;
// pub mod bitcoin_cash;
// pub mod cardano;
pub mod checksum_address;
// pub mod dogecoin;
// pub mod hedera;
// pub mod litecoin;
// pub mod monacoin;
// pub mod monero;
// pub mod p2pkh;
// pub mod p2sh;
// pub mod polkadot;
// pub mod ripple;
// pub mod segwit;
// pub mod solana;
// pub mod stellar;
// pub mod tezos;

#[derive(Debug, Error)]
pub enum MulticoinEncoderError {
    #[error("Invalid Structure {0}")]
    InvalidStructure(String),

    #[error("Not supported")]
    NotSupported,
}

pub trait MulticoinEncoder {
    fn encode(&self, data: String) -> Result<Vec<u8>, MulticoinEncoderError>;
}

impl CoinType {
    pub fn encode(&self, data: String) -> Result<Vec<u8>, MulticoinEncoderError> {
        let encoder: Box<dyn MulticoinEncoder> = match self {
            Self::Slip44(slip44) => match slip44 {
                // SLIP44::Bitcoin => Box::new(BitcoinDecoder {}),
                SLIP44::Ethereum | SLIP44::EthereumClassic | SLIP44::Rootstock => {
                    Box::new(EvmEncoder {})
                }
                // SLIP44::Litecoin => Box::new(LitecoinDecoder {}),
                // SLIP44::BitcoinCash => Box::new(BitcoinCashDecoder {}),
                // SLIP44::Solana => Box::new(SolanaDecoder {}),
                // SLIP44::Hedera => Box::new(HederaDecoder {}),
                // SLIP44::Stellar => Box::new(StellarDecoder {}),
                // SLIP44::Dogecoin => Box::new(DogecoinDecoder {}),
                // SLIP44::Monacoin => Box::new(MonacoinDecoder {}),
                // SLIP44::Monero => Box::new(MoneroDecoder {}),
                // SLIP44::Ripple => Box::new(RippleDecoder {}),
                // SLIP44::Tezos => Box::new(TezosDecoder {}),
                // SLIP44::Cardano => Box::new(CardanoDecoder {}),
                // SLIP44::Binance => Box::new(BinanceDecoder {}),
                // SLIP44::Polkadot => Box::new(PolkadotDecoder {}),
                _ => return Err(MulticoinEncoderError::NotSupported),
            },
            Self::Evm => Box::new(EvmEncoder {}),
        };

        encoder.encode(data)
    }
}
