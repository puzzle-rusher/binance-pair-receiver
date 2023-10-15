mod error;
mod model;
mod simple_rest_client;

use crate::error::Error;
use crate::model::request::PairsRequest;
use crate::model::response::SymbolStatus;
use crate::simple_rest_client::SimpleBinanceRestClient;
use std::time::Duration;
use tokio::io::AsyncWriteExt;

const BINANCE_URL_STR: &str = "https://api.binance.com";
const PERIOD_BETWEEN_RETRIES_IN_SECONDS: u64 = 10;

#[tokio::main]
async fn main() -> Result<(), Error> {
    inner_main().await?;
    flush_tokio_output().await?;

    Ok(())
}

async fn inner_main() -> Result<(), Error> {
    let rest_client = SimpleBinanceRestClient::new(BINANCE_URL_STR)?;

    let mut interval =
        tokio::time::interval(Duration::from_secs(PERIOD_BETWEEN_RETRIES_IN_SECONDS));

    loop {
        interval.tick().await;

        match rest_client.request(PairsRequest::new()).await {
            Ok(object) => {
                for symbol in object
                    .symbols
                    .into_iter()
                    .filter(|symbol| matches!(symbol.status, SymbolStatus::Trading))
                {
                    print_async(format!("{}/{}\n", symbol.base_asset, symbol.quote_asset)).await?;
                }
                break;
            }
            Err(err) => {
                print_async(format!(
                    "Error occured: {}\n Request attempt will be repeated within {} seconds",
                    err, PERIOD_BETWEEN_RETRIES_IN_SECONDS
                ))
                .await?;
            }
        };
    }

    Ok(())
}

async fn print_async(string: impl AsRef<str>) -> tokio::io::Result<()> {
    tokio::io::stdout()
        .write_all(string.as_ref().as_bytes())
        .await
}

async fn flush_tokio_output() -> tokio::io::Result<()> {
    tokio::io::stdout().flush().await
}
