use ethers::abi::ParamType;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolverFunctionCall {
    // addr(bytes32 node) returns (address)
    // 0x3b3b57de
    Addr(Vec<u8>),
    // name(bytes32 node) returns (string)
    // 0x691f3431
    Name(Vec<u8>),
    // abi(bytes32 node, uint256 contentTypes) returns (uint256, bytes)
    // 0x2203ab56
    Abi,
    // text(bytes32 node, string key) returns (string)
    // 0x59d1d43c
    Text(Vec<u8>, String),
    // contenthash(bytes32 node) returns (bytes)
    // 0xbc1c58d1
    ContentHash,
    // interfaceImplementer(bytes32 node, bytes4 interfaceID) returns (address)
    // 0xb8f2bbb4
    InterfaceImplementer,
    // addr(bytes32 node, uint256 coinType) returns (bytes)
    // 0xf1cb7e06
    AddrMultichain(Vec<u8>, u64),
    // pubkey(bytes32 node) returns (bytes32, bytes32)
    // 0xc8690233
    PubKey,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResolverFunctionCallDecodingError {
    InvalidSelector(String),
    InvalidPayload,
    InvalidNamehash,
    ABIDecodeError,
}

impl TryFrom<&[u8]> for ResolverFunctionCall {
    type Error = ResolverFunctionCallDecodingError;

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        let selector: &[u8; 4] = data[0..4].try_into().expect("Array length is correct");
        let selector_hex = hex::encode(selector);

        let payload = &data[4..];

        match selector_hex.as_str() {
            "3b3b57de" => {
                let result = ethers::abi::decode(&[ParamType::FixedBytes(32)], payload)
                    .map_err(|_| ResolverFunctionCallDecodingError::ABIDecodeError)?;
                let namehash = result[0]
                    .clone()
                    .into_fixed_bytes()
                    .ok_or(ResolverFunctionCallDecodingError::InvalidNamehash)?;

                Ok(ResolverFunctionCall::Addr(namehash))
            }
            "691f3431" => {
                let result = ethers::abi::decode(&[ParamType::FixedBytes(32)], payload).unwrap();
                let namehash = result[0].clone().into_fixed_bytes().unwrap();

                Ok(ResolverFunctionCall::Name(namehash))
            }
            "2203ab56" => Ok(ResolverFunctionCall::Abi),
            "59d1d43c" => {
                let result =
                    ethers::abi::decode(&[ParamType::FixedBytes(32), ParamType::String], payload)
                        .unwrap();
                let namehash = result[0].clone().into_fixed_bytes().unwrap();
                let record = result[1].clone().into_string().unwrap();

                Ok(ResolverFunctionCall::Text(namehash, record))
            }
            "bc1c58d1" => Ok(ResolverFunctionCall::ContentHash),
            "b8f2bbb4" => Ok(ResolverFunctionCall::InterfaceImplementer),
            "f1cb7e06" => {
                let result =
                    ethers::abi::decode(&[ParamType::FixedBytes(32), ParamType::Uint(64)], payload)
                        .unwrap();
                let namehash = result.get(0).unwrap().clone().into_fixed_bytes().unwrap();
                let coin_type: u64 = result.get(1).unwrap().clone().into_uint().unwrap().as_u64();

                Ok(ResolverFunctionCall::AddrMultichain(namehash, coin_type))
            }
            "c8690233" => Ok(ResolverFunctionCall::PubKey),
            _ => Err(ResolverFunctionCallDecodingError::InvalidSelector(
                selector_hex,
            )),
        }
    }
}
