use super::{MulticoinEncoder, MulticoinEncoderError};

pub struct EvmEncoder {}

impl MulticoinEncoder for EvmEncoder {
    fn encode(&self, data: String) -> Result<Vec<u8>, MulticoinEncoderError> {
        ethers::utils::hex::decode(data)
            .map_err(|err| MulticoinEncoderError::InvalidStructure(err.to_string()))
    }
}
