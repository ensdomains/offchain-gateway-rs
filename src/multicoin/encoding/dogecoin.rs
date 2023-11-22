// use super::{p2pkh::P2PKHDecoder, p2sh::P2SHDecoder, MulticoinEncoder, MulticoinEncoderError};
//
// pub struct DogecoinDecoder {}
//
// impl MulticoinEncoder for DogecoinDecoder {
//     fn encode(&self, data: &[u8]) -> Result<String, MulticoinEncoderError> {
//         match data.len() {
//             25 => P2PKHDecoder { version: 0x1e }.encode(data),
//             23 => P2SHDecoder { version: 0x16 }.encode(data),
//             _ => Err(MulticoinEncoderError::InvalidStructure(String::new())),
//         }
//     }
// }
