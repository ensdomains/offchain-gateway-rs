use std::{collections::HashMap, env};

use tokio_postgres::NoTls;
use tracing::{error, info};

pub struct Database {
    pub client: tokio_postgres::Client,
}

/// Connects to database
pub async fn bootstrap() -> Database {
    info!("Bootstrapping the database...");
    let (client, connection) = tokio_postgres::connect(
        &format!(
            "host={} user={} password={}",
            env::var("POSTGRES_HOST").unwrap_or("localhost".to_string()),
            env::var("POSTGRES_USER").unwrap_or("postgres".to_string()),
            env::var("POSTGRES_PASSWORD").unwrap_or("example".to_string()),
        ),
        NoTls,
    )
    .await
    .unwrap();

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            error!("connection error: {}", e);
        }
    });

    info!("Creating the database...");
    // Now we can execute a simple statement that just returns its parameter.
    // let rows = client.query("SELECT $1::TEXT", &[&"hello world"]).await?;

    client
        .batch_execute("CREATE EXTENSION IF NOT EXISTS hstore;")
        .await
        .unwrap();

    client.batch_execute("CREATE TABLE IF NOT EXISTS ens_data (node BYTEA PRIMARY KEY, records HSTORE, addresses HSTORE);").await.unwrap();

    let total_rows = client
        .query("SELECT COUNT(*) FROM ens_data", &[])
        .await
        .unwrap();

    info!("Total rows: {}", total_rows[0].get::<_, i64>(0));

    Database { client }
}

impl Database {
    pub async fn upsert(
        &self,
        node: &Vec<u8>,
        records: &HashMap<String, Option<String>>,
        addresses: &HashMap<String, Option<String>>,
    ) {
        self.client
            .execute(
                "INSERT INTO ens_data (node, records, addresses) VALUES ($1, $2, $3) ON CONFLICT (node) DO UPDATE SET records = $2, addresses = $3",
                &[node, records, addresses],
            )
            .await
            .unwrap();
    }

    pub async fn get_records(
        &self,
        node: &[u8],
        records: &[&str],
    ) -> HashMap<String, Option<String>> {
        // require that every record matches /a-zA-Z\./
        // if records.iter().any(|x| !x.chars().all(|c| c.is_alphanumeric() || c == '.')) {
        //     panic!("Invalid record name");
        // }

        // converts ['avatar', 'header'] to "records->'avatar', records->'header'"
        let records_raw = records.iter().fold(String::new(), |acc, x| {
            if acc.is_empty() {
                format!("records->'{}'", x)
            } else {
                format!("{}, records->'{}'", acc, x)
            }
        });

        let x = self
            .client
            .query_one(
                &format!("SELECT {} FROM ens_data WHERE node = $1", records_raw),
                &[&node],
            )
            .await
            .unwrap();

        records
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut map, (i, record)| {
                map.insert(record.to_string(), x.get::<_, Option<String>>(i));
                map
            })
    }

    pub async fn get_addresses(
        &self,
        node: &[u8],
        addresses: &[&str],
    ) -> HashMap<String, Option<String>> {
        // require that every record matches /a-zA-Z\./
        // if records.iter().any(|x| !x.chars().all(|c| c.is_alphanumeric() || c == '.')) {
        //     panic!("Invalid record name");
        // }

        // converts ['avatar', 'header'] to "records->'avatar', records->'header'"
        let addresses_raw = addresses.iter().fold(String::new(), |acc, x| {
            if acc.is_empty() {
                format!("addresses->'{}'", x)
            } else {
                format!("{}, addresses->'{}'", acc, x)
            }
        });

        let x = self
            .client
            .query_one(
                &format!("SELECT {} FROM ens_data WHERE node = $1", addresses_raw),
                &[&node],
            )
            .await
            .unwrap();

        addresses
            .iter()
            .enumerate()
            .fold(HashMap::new(), |mut map, (i, record)| {
                map.insert(record.to_string(), x.get::<_, Option<String>>(i));
                map
            })
    }

    pub async fn get_all(&self, node: &[u8]) -> (HashMap<String, Option<String>>, HashMap<String, Option<String>>) {
        let x = self
            .client
            .query_one(
                "SELECT records, addresses FROM ens_data WHERE node = $1",
                &[&node],
            )
            .await
            .unwrap();

        let records = x.get::<_, HashMap<String, Option<String>>>(0);
        let addresses = x.get::<_, HashMap<String, Option<String>>>(1);

        (records, addresses)
    }
}
