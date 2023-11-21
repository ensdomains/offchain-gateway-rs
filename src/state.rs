use ethers::signers::LocalWallet;

use crate::database::Database;

pub struct GlobalState {
    pub db: Database,
    pub wallet: LocalWallet,
}
