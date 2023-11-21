use std::{collections::HashMap, sync::Arc};

use axum::response::IntoResponse;
use ethers::{abi::Token, providers::namehash, utils::keccak256};
use tracing::info;

use crate::{ccip::lookup::ResolverFunctionCall, state::GlobalState};

use super::{payload::ResolveCCIPPostPayload, response::GatewayResponse, signing::UnsignedPayload};

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
                let v = hex::encode(_bf);

                info!("Resolving text record: {:?}, for {}", v, self.name);

                let hash = namehash(&self.name).to_fixed_bytes().to_vec();

                info!("Resolving text record: {:?}", hash);

                let x = state.db.get_records(&hash, &["avatar"]).await;
                info!("Resolving text recordz: {:?}", x);

                // state
                //     .db
                //     .upsert(&hash, &HashMap::default(), &HashMap::default())
                //     .await;

                // let str = String::from_utf8(_bf.clone()).unwrap();

                // info!("Resolving text record: {:?}", str);

                // state.db.get_records(node, records)
                vec![Token::String("Hello".to_string())]
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
