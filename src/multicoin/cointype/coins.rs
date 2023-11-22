use super::slip44::SLIP44;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoinType {
    Slip44(SLIP44),
    Evm,
}

impl From<u32> for CoinType {
    fn from(value: u32) -> CoinType {
        if value >= 0x8000_0000 {
            return CoinType::Evm;
        }

        SLIP44::from(value).into()
    }
}

#[cfg(test)]
mod tests {
    use super::super::slip44::SLIP44;
    use super::*;

    #[test]
    fn test_coin_type() {
        assert_eq!(CoinType::from(0), SLIP44::Bitcoin.into());
    }

    #[test]
    fn test_coin_type_evm() {
        assert_eq!(CoinType::from(2147483649), CoinType::Evm);
    }

    #[test]
    fn test_coin_type_evm_gnosis() {
        assert_eq!(CoinType::from(2147483748), CoinType::Evm);
    }
}
