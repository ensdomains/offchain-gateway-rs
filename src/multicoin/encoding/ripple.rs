use bs58::Alphabet;

use crate::utils;

use super::{MulticoinEncoder, MulticoinEncoderError};

pub struct RippleEncoder {}

impl MulticoinEncoder for RippleEncoder {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
        let decoded = bs58::decode(data)
            .with_alphabet(Alphabet::RIPPLE)
            .into_vec()
            .map_err(|_| {
                MulticoinEncoderError::InvalidStructure("failed to decode bs58".to_string())
            })?;

        // at least 1 byte of data and 4 bytes of checksum
        if decoded.len() < 5 {
            return Err(MulticoinEncoderError::InvalidStructure("".to_string()));
        }

        let checksum_begin = decoded.len() - 4;
        let checksum = &decoded[checksum_begin..];
        let data = &decoded[..checksum_begin];

        let checksum_check = &utils::sha256::hash(utils::sha256::hash(data))[..4];

        if checksum != checksum_check {
            return Err(MulticoinEncoderError::InvalidStructure(
                "invalid checksum".to_string(),
            ));
        }

        Ok(data.to_vec())
    }
}
