use std::{env, str::FromStr};

use dotenvy::dotenv;
use ethers::signers::{LocalWallet, Signer};
use tracing::{info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

pub mod ccip;
pub mod database;
pub mod gateway;
mod http;
pub mod multicoin;
pub mod selfservice;
pub mod state;
pub mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let filter = EnvFilter::new(format!("offchain_gateway={}", Level::DEBUG));

    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::DEBUG)
        .with_env_filter(filter)
        // completes the builder.
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let db = database::bootstrap().await;

    let wallet: LocalWallet = LocalWallet::from_str(
        env::var("PRIVATE_KEY")
            .expect("Could not find PRIVATE_KEY")
            .as_str(),
    )
    .unwrap();

    let address = format!("{:?}", wallet.address());
    info!("Signing with address: {}", address);

    let state = state::GlobalState { db, wallet };

    http::serve(state).await;

    info!("Shutting down");
}

// let mut records = HashMap::new();
// records.insert(
//     "avatar".to_string(),
//     Some(
//         "https://metadata.ens.domains/goerli/avatar/luc.myeth.id?timestamp=1700508402907"
//             .to_string(),
//     ),
// );
// let addresses = HashMap::new();
// // let h = hex::decode("0123456789ABCDEF0123456789ABCDEF").unwrap();
// let h = namehash("luc.myeth.id").to_fixed_bytes().to_vec();
// db.upsert(&h, &records, &addresses).await;
// let r = db
//     .get_records(&h, &vec!["avatar", "display", "header"])
//     .await;
// println!("{:?}", r);
