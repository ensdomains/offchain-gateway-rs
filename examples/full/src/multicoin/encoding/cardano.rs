use bech32::primitives::hrp::Hrp;
use bs58::Alphabet;
use ciborium::value::Integer;
use ciborium::Value;
use lazy_static::lazy_static;

use super::{MulticoinEncoder, MulticoinEncoderError};

lazy_static! {
    static ref ADA_HRP: Hrp = Hrp::parse_unchecked("addr");
}

pub struct CardanoEncoder {}

// None if invalid bryon address
fn encode_cardano_bryon(data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
    if !data.starts_with("Ae2") && !data.starts_with("Ddz") {
        return Err(MulticoinEncoderError::InvalidStructure(
            "invalid bryon address prefix".to_string(),
        ));
    }

    let decoded = bs58::decode(data)
        .with_alphabet(Alphabet::BITCOIN)
        .into_vec()
        .map_err(|_| {
            MulticoinEncoderError::InvalidStructure("failed to decode bs58".to_string())
        })?;

    let (Value::Tag(tag, data_raw), Value::Integer(checksum)) =
        ciborium::from_reader(decoded.as_slice()).map_err(|_| {
            MulticoinEncoderError::InvalidStructure("failed to cbor decode".to_string())
        })?
    else {
        return Err(MulticoinEncoderError::InvalidStructure(
            "invalid cbor structure".to_string(),
        ));
    };

    let Some(data) = data_raw.as_bytes() else {
        return Err(MulticoinEncoderError::InvalidStructure(
            "invalid cbor structure".to_string(),
        ));
    };

    let checksum_check = crc32fast::hash(data);

    if tag != 24 || checksum != Integer::from(checksum_check) {
        return Err(MulticoinEncoderError::InvalidStructure(
            "invalid cbor structure".to_string(),
        ));
    };

    Ok(data.clone())
}

fn encode_cardano_shelley(data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
    let (hrp, data) = bech32::decode(data).map_err(|_| {
        MulticoinEncoderError::InvalidStructure("failed to bech32 encode".to_string())
    })?;

    if hrp != *ADA_HRP {
        return Err(MulticoinEncoderError::InvalidStructure(
            "invalid bech32 address prefix".to_string(),
        ));
    }

    Ok(data)
}

impl MulticoinEncoder for CardanoEncoder {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
        encode_cardano_bryon(data).or_else(|_| encode_cardano_shelley(data))
    }
}
