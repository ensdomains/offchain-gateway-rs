use super::{MulticoinEncoder, MulticoinEncoderError};

pub struct HederaEncoder {}

impl MulticoinEncoder for HederaEncoder {
    fn encode(&self, data: &str) -> Result<Vec<u8>, MulticoinEncoderError> {
        let parts: Vec<&str> = data.split('.').collect();
        if parts.len() != 3 {
            return Err(MulticoinEncoderError::InvalidStructure(
                "invalid length".to_string(),
            ));
        }

        let (Ok(shard), Ok(realm), Ok(account)) = (
            parts[0].parse::<u32>(),
            parts[1].parse::<u64>(),
            parts[2].parse::<u64>(),
        ) else {
            return Err(MulticoinEncoderError::InvalidStructure("".to_string()));
        };

        let mut result = Vec::new();
        result.extend_from_slice(&shard.to_be_bytes());
        result.extend_from_slice(&realm.to_be_bytes());
        result.extend_from_slice(&account.to_be_bytes());

        Ok(result)
    }
}
