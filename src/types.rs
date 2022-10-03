use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Request<'a, T> {
    pub jsonrpc: &'a str,
    pub id: u16,
    pub method: &'a str,
    pub params: T,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthParams<'a> {
    pub grant_type: &'a str,
    pub client_id: &'a str,
    pub client_secret: &'a str,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BalanceParams {
    pub with_portfolio: bool,
}
