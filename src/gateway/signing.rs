use std::sync::Arc;

use ethers::{
    abi::{AbiEncode, Token},
    signers::Signer,
    types::{H160, U256, U64},
    utils::keccak256,
};

use crate::state::GlobalState;
use super::response::GatewayResponse;

pub struct UnsignedPayload {
    pub data: Vec<u8>,
    pub sender: H160,
    pub request_hash: Vec<u8>,
    pub result_hash: Vec<u8>,
    pub expires: u64,
}

#[derive(Debug)]
pub enum SignError {
    UnknownError(String),
}

impl UnsignedPayload {
    pub fn sign(self, state: Arc<GlobalState>) -> Result<GatewayResponse, SignError> {
        let encoded = ethers::abi::encode_packed(&[
            Token::Uint(U256::from(0x1900)),
            Token::Address(self.sender),
            Token::FixedBytes(U64::from(self.expires).0[0].to_be_bytes().to_vec()),
            Token::FixedBytes(self.request_hash),
            Token::FixedBytes(self.result_hash),
        ])
        .unwrap();

        let message_hash = keccak256(encoded);

        let signature: ethers::types::Signature =
            state.wallet.sign_hash(message_hash.into()).unwrap();

        let signature_r = signature.r.encode();
        let signature_s = signature.s.encode();
        let signature_v = vec![signature.v.try_into().unwrap()];

        let signature = [signature_r, signature_s, signature_v].concat();

        let pl = format!(
            "0x{}",
            hex::encode(ethers::abi::encode(
                vec![
                    Token::Bytes(self.data),
                    Token::Uint(U256::from(self.expires)),
                    Token::Bytes(signature),
                ]
                .as_slice()
            ))
        );

        Ok(GatewayResponse::Data(pl))
    }
}
