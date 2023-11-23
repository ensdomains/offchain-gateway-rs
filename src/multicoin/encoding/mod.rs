use thiserror::Error;

use crate::multicoin::encoding::binance::BinanceEncoder;
use crate::multicoin::encoding::bitcoin::BitcoinEncoder;
use crate::multicoin::encoding::cardano::CardanoEncoder;
use crate::multicoin::encoding::hedera::HederaEncoder;
use crate::multicoin::encoding::polkadot::PolkadotEncoder;
use crate::multicoin::encoding::ripple::RippleEncoder;
use crate::multicoin::encoding::solana::SolanaEncoder;
use crate::multicoin::encoding::stellar::StellarEncoder;

use super::cointype::{coins::CoinType, slip44::SLIP44};

use self::checksum_address::EvmEncoder;

pub mod binance;
pub mod bitcoin;
pub mod cardano;
pub mod checksum_address;
pub mod hedera;
pub mod p2pkh;
pub mod p2sh;
pub mod polkadot;
pub mod ripple;
pub mod segwit;
pub mod solana;
pub mod stellar;

#[derive(Debug, Error)]
pub enum MulticoinEncoderError {
    #[error("Invalid structure: {0}")]
    InvalidStructure(String),

    #[error("Not supported")]
    NotSupported,
}

pub trait MulticoinEncoder {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError>;
}

impl MulticoinEncoder for CoinType {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
        let encoder: Box<dyn MulticoinEncoder> = match self {
            Self::Slip44(slip44) => match slip44 {
                SLIP44::Ethereum | SLIP44::EthereumClassic | SLIP44::Rootstock => {
                    Box::new(EvmEncoder {})
                }
                SLIP44::Bitcoin => Box::new(BitcoinEncoder::new(Some("bc"), &[0x00], &[0x05])),
                SLIP44::Litecoin => {
                    Box::new(BitcoinEncoder::new(Some("ltc"), &[0x30], &[0x32, 0x05]))
                }
                SLIP44::Dogecoin => Box::new(BitcoinEncoder::new(None, &[0x1e], &[0x16])),
                SLIP44::Solana => Box::new(SolanaEncoder {}),
                SLIP44::Hedera => Box::new(HederaEncoder {}),
                SLIP44::Stellar => Box::new(StellarEncoder {}),
                SLIP44::Ripple => Box::new(RippleEncoder {}),
                SLIP44::Cardano => Box::new(CardanoEncoder {}),
                SLIP44::Binance => Box::new(BinanceEncoder {}),
                SLIP44::Polkadot => Box::new(PolkadotEncoder {}),
                _ => return Err(MulticoinEncoderError::NotSupported),
            },
            Self::Evm => Box::new(EvmEncoder {}),
        };

        encoder.encode(data)
    }
}
