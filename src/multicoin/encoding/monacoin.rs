// use super::{p2pkh::P2PKHDecoder, p2sh::P2SHDecoder, MulticoinEncoder, MulticoinEncoderError};
//
// pub struct MonacoinDecoder {}
//
// impl MulticoinEncoder for MonacoinDecoder {
//     fn encode(&self, data: &[u8]) -> Result<String, MulticoinEncoderError> {
//         match data.len() {
//             25 => P2PKHDecoder { version: 0x32 }.encode(data),
//             23 => P2SHDecoder { version: 0x05 }.encode(data),
//             _ => Err(MulticoinEncoderError::InvalidStructure(String::new())),
//         }
//     }
// }
