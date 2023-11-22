// use lazy_static::lazy_static;
//
// use crate::models::multicoin::decoding::segwit::SegWitDecoder;
//
// use super::{p2pkh::P2PKHDecoder, p2sh::P2SHDecoder, MulticoinEncoder, MulticoinEncoderError};
//
// lazy_static! {
//     static ref LTC_SEGWIT_DECODER: SegWitDecoder = SegWitDecoder {
//         human_readable_part: "ltc".to_string()
//     };
// }
//
// pub struct LitecoinDecoder {}
//
// impl MulticoinEncoder for LitecoinDecoder {
//     fn encode(&self, data: &[u8]) -> Result<String, MulticoinEncoderError> {
//         if let Ok(address) = LTC_SEGWIT_DECODER.decode(data) {
//             return Ok(address);
//         }
//
//         if data.len() == 25 {
//             return P2PKHDecoder { version: 0x30 }.encode(data);
//         }
//
//         if data.len() == 23 {
//             return P2SHDecoder { version: 0x32 }.encode(data);
//         }
//
//         Err(MulticoinEncoderError::InvalidStructure(String::new()))
//     }
// }
