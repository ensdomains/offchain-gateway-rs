use ethers::abi::ParamType;
use serde::Deserialize;
use tracing::info;

use crate::ccip::lookup::ResolverFunctionCall;

use super::{resolution::UnresolvedQuery, response::GatewayResponse};

#[derive(Deserialize, Debug)]
pub struct ResolveCCIPPostPayload {
    pub data: String,
    pub sender: String,
}

impl ResolveCCIPPostPayload {
    /// This function handles the initial decoding of the payload
    /// It returns the name and the resolver function call that needs to be resolved
    /// TODO: Implement error handling
    pub fn decode(&self) -> Result<UnresolvedQuery, GatewayResponse> {
        let data = self
            .data
            .strip_prefix("0x9061b923")
            .expect("Prefix is not correct, invld request");
        let data = hex::decode(data).expect("Failed to decode to hex, invld request");

        let decoded = ethers::abi::decode(&[ParamType::Bytes, ParamType::Bytes], &data)
            .expect("Failed to abi decode, invld request");

        let dns_encoded_name = decoded[0]
            .clone()
            .into_bytes()
            .expect("Failed to decode bytes, invld request");

        let name =
            String::from_utf8(dns_encoded_name).expect("Failed to decode utf8, invld request");

        let name = crate::utils::dns::decode(&name);

        info!("Decoded name: {}", name);

        let rest_of_the_data = decoded[1]
            .clone()
            .into_bytes()
            .expect("Failed to decode bytes, invld request");

        Ok(UnresolvedQuery {
            name,
            data: ResolverFunctionCall::try_from(rest_of_the_data.as_slice())
                .expect("Failed to decode resolver function call, invld request"),
            calldata: self,
        })
    }
}
