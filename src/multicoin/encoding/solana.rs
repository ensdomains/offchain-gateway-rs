use super::{MulticoinEncoder, MulticoinEncoderError};

pub struct SolanaEncoder {}

impl MulticoinEncoder for SolanaEncoder {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
        bs58::decode(data).into_vec().map_err(|_| {
            MulticoinEncoderError::InvalidStructure("failed to decode bs58".to_string())
        })
    }
}
