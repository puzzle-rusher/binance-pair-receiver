use crate::error::Error;
use crate::model::request::BinanceRequest;
use reqwest::{StatusCode, Url};
use serde::de::DeserializeOwned;
use std::str::FromStr;

type Result<T> = std::result::Result<T, Error>;

pub(crate) struct SimpleBinanceRestClient {
    base_url: Url,
}

impl SimpleBinanceRestClient {
    pub(crate) fn new(url_str: &str) -> Result<SimpleBinanceRestClient> {
        let url = Url::from_str(url_str).map_err(|err| Error::ParseUrl(err.to_string()))?;

        Ok(Self { base_url: url })
    }

    pub(crate) async fn request<T: BinanceRequest>(&self, _: T) -> Result<T::Response> {
        let endpoint = T::endpoint();
        let mut url = self.base_url.clone();
        url.set_path(endpoint.as_ref());

        let response = reqwest::get(url).await.map_err(Error::Request)?;

        response_handler(response).await
    }
}

async fn response_handler<T: DeserializeOwned>(response: reqwest::Response) -> Result<T> {
    let status = response.status();
    let response_text = response
        .text()
        .await
        .map_err(|err| Error::ResponseParsingError(err.to_string()))?;
    match status {
        StatusCode::OK => {
            let response_object = serde_json::from_str(&response_text)
                .map_err(|err| Error::ResponseParsingError(err.to_string()))?;
            Ok(response_object)
        }
        status => Err(Error::UnknownResponse(format!(
            "Unsupported status: {status:?}",
        ))),
    }
}
