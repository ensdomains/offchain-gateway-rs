use base32::Alphabet;
use crc16::{State, XMODEM};

use super::{MulticoinEncoder, MulticoinEncoderError};

pub struct StellarEncoder {}

impl MulticoinEncoder for StellarEncoder {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
        let decoded = base32::decode(Alphabet::RFC4648 { padding: false }, data).ok_or(
            MulticoinEncoderError::InvalidStructure("failed to decode base32".to_string()),
        )?;

        // ed25519 version byte, at least 1 byte of data and 2 bytes of a hash
        if decoded.len() < 4 {
            return Err(MulticoinEncoderError::InvalidStructure("".to_string()));
        }

        let hash_begin = decoded.len() - 2;
        let checksum_bytes = &decoded[hash_begin..];
        let checksum = u16::from_le_bytes([checksum_bytes[0], checksum_bytes[1]]);
        let data = &decoded[1..hash_begin];

        let checksum_check = State::<XMODEM>::calculate(&decoded[..hash_begin]);

        if checksum != checksum_check {
            return Err(MulticoinEncoderError::InvalidStructure(
                "invalid checksum".to_string(),
            ));
        }

        Ok(data.to_vec())
    }
}
