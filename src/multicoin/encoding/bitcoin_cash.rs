// use super::{p2pkh::P2PKHDecoder, p2sh::P2SHDecoder, MulticoinEncoder, MulticoinEncoderError};
//
// pub struct BitcoinCashDecoder {}
//
// impl MulticoinEncoder for BitcoinCashDecoder {
//     fn encode(&self, data: &[u8]) -> Result<String, MulticoinEncoderError> {
//         if data.len() == 25 {
//             return P2PKHDecoder { version: 0x00 }.encode(data);
//         }
//
//         if data.len() == 23 {
//             return P2SHDecoder { version: 0x05 }.encode(data);
//         }
//
//         Err(MulticoinEncoderError::InvalidStructure(String::new()))
//     }
// }
