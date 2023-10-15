use crate::model::response::PairsResponse;
use serde::de::DeserializeOwned;
use std::borrow::Cow;

pub(crate) trait BinanceRequest {
    type Response: DeserializeOwned;

    fn endpoint() -> Cow<'static, str>;
}

#[derive(Debug, Clone)]
pub(crate) struct PairsRequest {}

impl PairsRequest {
    pub(crate) fn new() -> PairsRequest {
        PairsRequest {}
    }
}

impl BinanceRequest for PairsRequest {
    type Response = PairsResponse;

    fn endpoint() -> Cow<'static, str> {
        Cow::Borrowed("/api/v3/exchangeInfo")
    }
}
