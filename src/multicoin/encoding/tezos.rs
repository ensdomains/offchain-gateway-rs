// use bs58::Alphabet;
//
// use crate::utils;
//
// use super::{MulticoinEncoder, MulticoinEncoderError};
//
// pub struct TezosDecoder {}
//
// const CONTRACT_PREFIX: &[u8; 3] = &[0x02, 0x5a, 0x79];
//
// impl MulticoinEncoder for TezosDecoder {
//     fn encode(&self, data: &[u8]) -> Result<String, MulticoinEncoderError> {
//         if data.len() != 21 && data.len() != 22 {
//             return Err(MulticoinEncoderError::InvalidStructure(
//                 "invalid address length".to_string(),
//             ));
//         }
//
//         let prefix: &[u8; 3] = match data[0] {
//             0x00 => match data[1] {
//                 0x00 => Ok(&[0x06, 0xa1, 0x9f]),
//                 0x01 => Ok(&[0x06, 0xa1, 0xa1]),
//                 0x02 => Ok(&[0x06, 0xa1, 0xa4]),
//                 0x03 => Ok(&[0x06, 0xa1, 0xa6]),
//                 _ => Err(MulticoinEncoderError::InvalidStructure(
//                     "invalid address format".to_string(),
//                 )),
//             },
//             0x01 => Ok(CONTRACT_PREFIX),
//             _ => Err(MulticoinEncoderError::InvalidStructure(
//                 "invalid address type".to_string(),
//             )),
//         }?;
//
//         let decoded = [
//             prefix as &[u8],
//             match prefix {
//                 CONTRACT_PREFIX => &data[1..data.len() - 1],
//                 _ => &data[2..],
//             },
//         ]
//         .concat();
//
//         let checksum = utils::sha256::hash(utils::sha256::hash(decoded.clone()));
//
//         Ok(bs58::encode([&decoded, &checksum[..4]].concat())
//             .with_alphabet(Alphabet::BITCOIN)
//             .into_string())
//     }
// }
//
// // TODO: tests
