use ethers::abi::ParamType;
use serde::Deserialize;
use thiserror::Error;
use tracing::info;

use crate::ccip::lookup::{ResolverFunctionCall, ResolverFunctionCallDecodingError};

use super::resolution::UnresolvedQuery;

#[derive(Deserialize, Debug)]
pub struct ResolveCCIPPostPayload {
    pub data: String,
    pub sender: String,
}

#[derive(Debug, Error)]
pub enum ResolverDecodeError {
    #[error("Invalid prefix")]
    InvalidPrefix,
    #[error("Invalid hex")]
    InvalidHex(#[from] hex::FromHexError),
    #[error("Invalid utf8")]
    InvalidUtf8(#[from] std::string::FromUtf8Error),
    #[error("Invalid abi")]
    InvalidAbi(#[from] ethers::abi::Error),
    #[error("Invalid bytes")]
    InvalidBytes,
    #[error("Resolver Function Call")]
    ResolverFunctionCall(#[from] ResolverFunctionCallDecodingError),
}

impl ResolveCCIPPostPayload {
    /// This function handles the initial encoding of the payload
    /// It returns the name and the resolver function call that needs to be resolved
    /// TODO: Implement error handling
    pub fn decode(&self) -> Result<UnresolvedQuery, ResolverDecodeError> {
        let data = self
            .data
            .strip_prefix("0x9061b923")
            .ok_or(ResolverDecodeError::InvalidPrefix)?;

        let data = hex::decode(data)?;

        let decoded = ethers::abi::decode(&[ParamType::Bytes, ParamType::Bytes], &data)?;

        let dns_encoded_name = decoded[0]
            .clone()
            .into_bytes()
            .ok_or(ResolverDecodeError::InvalidBytes)?;

        let name = String::from_utf8(dns_encoded_name)?;

        let name = crate::utils::dns::decode(&name);

        info!("Decoded name: {}", name);

        let rest_of_the_data = decoded[1]
            .clone()
            .into_bytes()
            .ok_or(ResolverDecodeError::InvalidBytes)?;

        let data = ResolverFunctionCall::try_from(rest_of_the_data.as_slice())?;

        Ok(UnresolvedQuery {
            name,
            data,
            calldata: self,
        })
    }
}
