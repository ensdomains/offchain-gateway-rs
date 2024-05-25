use bech32::primitives::hrp::Hrp;
use lazy_static::lazy_static;

use super::{MulticoinEncoder, MulticoinEncoderError};

lazy_static! {
    static ref BNB_HRP: Hrp = Hrp::parse_unchecked("bnb");
}

pub struct BinanceEncoder {}

impl MulticoinEncoder for BinanceEncoder {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
        let (hrp, data) = bech32::decode(data).map_err(|_| {
            MulticoinEncoderError::InvalidStructure("failed to decode bech32".to_string())
        })?;

        if hrp != *BNB_HRP {
            return Err(MulticoinEncoderError::InvalidStructure(
                "invalid binance hrp".to_string(),
            ));
        }

        Ok(data)
    }
}
