use bech32::Hrp;

use crate::multicoin::encoding::{MulticoinEncoder, MulticoinEncoderError};

pub struct SegWitEncoder {
    pub human_readable_part: Hrp,
}

impl SegWitEncoder {
    pub fn new(human_readable_part: &str) -> SegWitEncoder {
        SegWitEncoder {
            human_readable_part: Hrp::parse_unchecked(human_readable_part),
        }
    }
}

impl MulticoinEncoder for SegWitEncoder {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
        let (hrp, version, data) = bech32::segwit::decode(data).map_err(|_| {
            MulticoinEncoderError::InvalidStructure("failed to bech32 decode".to_string())
        })?;

        if hrp != self.human_readable_part {
            return Err(MulticoinEncoderError::InvalidStructure(
                "invalid segwit prefix".to_string(),
            ));
        }

        let version_u8 = version.to_u8();
        let version = match version_u8 {
            0x00 => Ok(0x00),
            0x01..=0x10 => Ok(version_u8 + 0x50),
            _ => Err(MulticoinEncoderError::InvalidStructure(
                "invalid segwit version".to_string(),
            )),
        }?;

        Ok([&[version, data.len() as u8] as &[u8], &data]
            .concat()
            .to_vec())
    }
}
