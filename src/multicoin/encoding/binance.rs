// use bech32::primitives::hrp::Hrp;
// use bech32::Bech32;
//
// use super::{MulticoinEncoder, MulticoinEncoderError};
//
// pub struct BinanceDecoder {}
//
// impl MulticoinEncoder for BinanceDecoder {
//     fn encode(&self, data: &[u8]) -> Result<String, MulticoinEncoderError> {
//         bech32::encode::<Bech32>(Hrp::parse_unchecked("bnb"), data).map_err(|_| {
//             MulticoinEncoderError::InvalidStructure("failed to bech32 encode".to_string())
//         })
//     }
// }
