use crate::types::{AuthParams, BalanceParams, Request};
use serde::Serialize;
use serde_json::Value;
use std::net::TcpStream;
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, Result, WebSocket};
use url::Url;

#[derive(Debug, Clone)]
struct Creds<'a> {
    drb_public: &'a str,
    drb_private: &'a str,
}

#[derive(Debug)]
pub struct Dealer<'a> {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
    creds: Creds<'a>,
}

impl<'a> Dealer<'a> {
    pub fn new(drb_wss: &str, drb_public: &'a str, drb_private: &'a str) -> Self {
        let (socket, _) =
            connect(Url::parse(drb_wss).unwrap()).expect("Could not connect to Deribit");

        Dealer {
            socket,
            creds: Creds {
                drb_public,
                drb_private,
            },
        }
    }

    pub fn send<T: Serialize>(&mut self, msg: T) -> Result<()> {
        let message = serde_json::to_string(&msg).unwrap();
        self.socket.write_message(Message::Text(message))
    }

    pub fn read(&mut self) -> Result<Value> {
        let msg = self.socket.read_message()?;

        let msg = match msg {
            Message::Text(s) => s,
            _ => panic!(), // change this to something informative
        };

        let parsed: Value = serde_json::from_str(&msg).expect("Can't parse to JSON");

        Ok(parsed)
    }

    pub fn authenicate(&mut self) -> Result<Value, tungstenite::Error> {
        let auth_params = AuthParams {
            grant_type: "client_credentials",
            client_id: self.creds.drb_public,
            client_secret: self.creds.drb_private,
        };

        let auth = Request {
            jsonrpc: "2.0",
            id: 9929,
            method: "public/auth",
            params: auth_params,
        };

        self.send(&auth)?;
        self.read()
    }

    pub fn get_balance(&mut self) -> Result<Value, tungstenite::Error> {
        let balance_params = BalanceParams {
            with_portfolio: true,
        };

        let balance = Request {
            jsonrpc: "2.0",
            id: 4947,
            method: "private/get_subaccounts",
            params: balance_params,
        };

        self.send(&balance)?;
        self.read()
    }
}
