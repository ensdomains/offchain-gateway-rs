// use super::{MulticoinEncoder, MulticoinEncoderError};
//
// pub struct HederaDecoder {}
//
// impl MulticoinEncoder for HederaDecoder {
//     fn encode(&self, data: &[u8]) -> Result<String, MulticoinEncoderError> {
//         if data.len() != 20 {
//             return Err(MulticoinEncoderError::InvalidStructure(String::new()));
//         }
//
//         let shard = u32::from_be_bytes((&data[..4]).try_into().unwrap());
//         let realm = u64::from_be_bytes((&data[4..12]).try_into().unwrap());
//         let account = u64::from_be_bytes((&data[12..]).try_into().unwrap());
//
//         Ok(format!("{shard}.{realm}.{account}"))
//     }
// }
