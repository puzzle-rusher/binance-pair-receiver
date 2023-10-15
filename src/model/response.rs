use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub(crate) struct PairsResponse {
    pub(crate) symbols: Vec<PairSymbol>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct PairSymbol {
    pub(crate) status: SymbolStatus,
    pub(crate) base_asset: String,
    pub(crate) quote_asset: String,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub(crate) enum SymbolStatus {
    PreTrading,
    Trading,
    PostTrading,
    EndOfDay,
    Halt,
    AuctionMatch,
    Break,
}
