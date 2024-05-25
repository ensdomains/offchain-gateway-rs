use blake2::{Blake2b512, Digest};
use bs58::Alphabet;

use super::{MulticoinEncoder, MulticoinEncoderError};

pub struct PolkadotEncoder {}

static HASH_PREFIX: &[u8] = b"SS58PRE";

impl MulticoinEncoder for PolkadotEncoder {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
        let decoded = bs58::decode(data)
            .with_alphabet(Alphabet::BITCOIN)
            .into_vec()
            .map_err(|_| {
                MulticoinEncoderError::InvalidStructure("failed to decode bs58".to_string())
            })?;

        // null byte, at least 1 byte of data and 2 bytes of a hash
        if decoded.len() < 4 {
            return Err(MulticoinEncoderError::InvalidStructure("".to_string()));
        }

        let hash_begin = decoded.len() - 2;
        let hash = &decoded[hash_begin..];
        let data = &decoded[1..hash_begin];

        let mut hasher = Blake2b512::new();
        hasher.update([HASH_PREFIX, &[0], data].concat());

        let check_hash = hasher.finalize();

        if hash != &check_hash.as_slice()[..2] {
            return Err(MulticoinEncoderError::InvalidStructure(
                "invalid checksum".to_string(),
            ));
        }

        Ok(data.to_vec())
    }
}
