use dotenvy::dotenv;
use tracing::info;

pub mod gateway;
pub mod database;
mod http;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt().init();

    let db = database::bootstrap().await;

    http::serve(db).await;

    info!("Shutting down");

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
}
