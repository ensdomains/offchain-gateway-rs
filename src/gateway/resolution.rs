use std::sync::Arc;

use axum::response::IntoResponse;
use ethers::{utils::keccak256, abi::Token};
use tracing::info;

use crate::{ccip::lookup::ResolverFunctionCall, state::GlobalState};

use super::{response::GatewayResponse, signing::UnsignedPayload, payload::ResolveCCIPPostPayload};

pub struct UnresolvedQuery<'a> {
    pub name: String,
    pub data: ResolverFunctionCall,
    pub calldata: &'a ResolveCCIPPostPayload,
}

impl UnresolvedQuery<'_> {
    pub async fn resolve(
        &self,
        state: Arc<GlobalState>,
    ) -> Result<UnsignedPayload, GatewayResponse> {
        info!("Resolving query: {:?}", self.data);

        let payload: Vec<Token> = match &self.data {
            ResolverFunctionCall::Text(_bf, record) => {
                vec![
                    Token::String("Hello".to_string()),
                ]
            }
            _ => {
                Vec::new()
            }
        };

        let expires = 1703980800; //chrono::Utc::now().timestamp() as u64 + 3600;
        let sender = self.calldata.sender.parse().unwrap();

        let data = ethers::abi::encode(&payload);
        let request_hash = keccak256(&self.calldata.data).to_vec();
        let result_hash = keccak256(&data).to_vec();

        Ok(UnsignedPayload {
            data,
            expires,
            request_hash,
            result_hash,
            sender,
        })
    }
}
