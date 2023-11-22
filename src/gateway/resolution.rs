use std::sync::Arc;

use ethers::{abi::Token, providers::namehash, utils::keccak256, types::H160};
use thiserror::Error;
use tracing::info;

use crate::{ccip::lookup::ResolverFunctionCall, state::GlobalState};

use super::{payload::ResolveCCIPPostPayload, signing::UnsignedPayload};

pub struct UnresolvedQuery<'a> {
    pub name: String,
    pub data: ResolverFunctionCall,
    pub calldata: &'a ResolveCCIPPostPayload,
}

#[derive(Debug, Error)]
pub enum ResolveError {
    #[error("Unknown error")]
    NotFound,
}

impl UnresolvedQuery<'_> {
    pub async fn resolve(&self, state: Arc<GlobalState>) -> Result<UnsignedPayload, ResolveError> {
        info!("Resolving query: {:?}", self.data);

        let payload: Vec<Token> = match &self.data {
            ResolverFunctionCall::Text(_bf, record) => {
                let v = hex::encode(_bf);

                info!("Resolving text record: {:?}, for {}", v, self.name);

                let hash = namehash(&self.name).to_fixed_bytes().to_vec();

                info!("Resolving text record: {:?}", hash);

                let x = state.db.get_records(&hash, &[record]).await;

                let value = x.get(record).to_owned().unwrap().clone().unwrap();

                vec![Token::String(value)]
            }
            ResolverFunctionCall::AddrMultichain(_bf, chain) => {
                let hash = namehash(&self.name).to_fixed_bytes().to_vec();

                info!("Resolving addr record: {:?}", hash);

                let x = state.db.get_addresses(&hash, &[&chain.to_string()]).await;

                let value = x.get(&chain.to_string()).to_owned().unwrap().clone().unwrap();

                let bytes = value.as_bytes().to_vec();

                vec![Token::Bytes(bytes)]
            }
            ResolverFunctionCall::Addr(_bf) => {
                let chain = 60;
                let hash = namehash(&self.name).to_fixed_bytes().to_vec();

                info!("Resolving addr record: {:?}", hash);

                let x = state.db.get_addresses(&hash, &[&chain.to_string()]).await;

                let value = x.get(&chain.to_string()).to_owned().unwrap().clone().unwrap();

                let address = value.parse().unwrap();

                vec![Token::Address(address)]
            }
            _ => Vec::new(),
        };

        let ttl = 3600;
        let expires = chrono::Utc::now().timestamp() as u64 + ttl;
        let sender = self.calldata.sender.parse().unwrap();
        let request_payload = hex::decode(self.calldata.data.trim_start_matches("0x")).unwrap();
        let data = ethers::abi::encode(&payload);
        let request_hash = keccak256(request_payload).to_vec();
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
