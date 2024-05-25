use bs58::Alphabet;

use crate::utils;

use super::{MulticoinEncoder, MulticoinEncoderError};

pub struct P2PKHEncoder {
    pub(crate) accepted_versions: &'static [u8],
}

impl MulticoinEncoder for P2PKHEncoder {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
        let decoded = bs58::decode(data)
            .with_alphabet(Alphabet::BITCOIN)
            .into_vec()
            .map_err(|_| {
                MulticoinEncoderError::InvalidStructure("failed to decode bs58".to_string())
            })?;

        // version byte, at least one data byte, 4 bytes of checksum
        if decoded.len() < 6 {
            return Err(MulticoinEncoderError::InvalidStructure("".to_string()));
        }

        if !self
            .accepted_versions
            .iter()
            .any(|version| decoded[0] == *version)
        {
            return Err(MulticoinEncoderError::InvalidStructure(
                "invalid version".to_string(),
            ));
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

        let pub_key_hash = &data[1..];

        Ok([
            &[0x76, 0xa9, pub_key_hash.len() as u8] as &[u8],
            pub_key_hash,
            &[0x88, 0xac],
        ]
        .concat()
        .to_vec())
    }
}
