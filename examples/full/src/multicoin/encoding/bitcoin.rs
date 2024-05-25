// use lazy_static::lazy_static;

use crate::multicoin::encoding::segwit::SegWitEncoder;

use super::{p2pkh::P2PKHEncoder, p2sh::P2SHEncoder, MulticoinEncoder, MulticoinEncoderError};

pub struct BitcoinEncoder {
    segwit_encoder: Option<SegWitEncoder>,
    p2pkh_encoder: P2PKHEncoder,
    p2sh_encoder: P2SHEncoder,
}

impl BitcoinEncoder {
    pub fn new(
        segwit_hrp: Option<&str>,
        p2pkh_versions: &'static [u8],
        p2sh_versions: &'static [u8],
    ) -> Self {
        Self {
            segwit_encoder: segwit_hrp.map(|hrp| SegWitEncoder::new(hrp)),
            p2pkh_encoder: P2PKHEncoder {
                accepted_versions: p2pkh_versions,
            },
            p2sh_encoder: P2SHEncoder {
                accepted_versions: p2sh_versions,
            },
        }
    }
}

impl MulticoinEncoder for BitcoinEncoder {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
        if let Some(segwit_encoder) = &self.segwit_encoder {
            if let Ok(address) = segwit_encoder.encode(data) {
                return Ok(address);
            }
        }

        self.p2pkh_encoder
            .encode(data)
            .or_else(|_| self.p2sh_encoder.encode(data))
            .map_err(|_| MulticoinEncoderError::InvalidStructure(String::new()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_btc_p2pkh() {
        let decoded = BitcoinEncoder::new(Some("bc"), &[0x00], &[0x05])
            .encode("1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa")
            .unwrap();

        assert_eq!(
            decoded,
            &hex_literal::hex!("76a91462e907b15cbf27d5425399ebf6f0fb50ebb88f1888ac")
        );
    }

    #[tokio::test]
    async fn test_btc_p2sh() {
        let decoded = BitcoinEncoder::new(Some("bc"), &[0x00], &[0x05])
            .encode("3Ai1JZ8pdJb2ksieUV8FsxSNVJCpoPi8W6")
            .unwrap();

        assert_eq!(
            decoded,
            &hex_literal::hex!("a91462e907b15cbf27d5425399ebf6f0fb50ebb88f1887")
        );
    }

    #[tokio::test]
    async fn test_btc_segwit() {
        let decoded = BitcoinEncoder::new(Some("bc"), &[0x00], &[0x05])
            .encode("bc1qw508d6qejxtdg4y5r3zarvary0c5xw7kv8f3t4")
            .unwrap();

        assert_eq!(
            decoded,
            &hex_literal::hex!("0014751e76e8199196d454941c45d1b3a323f1433bd6")
        );
    }
}
