use rpc_api::*;
use types::{ImplicitAddress, PrivateKey, PublicKey};

/// Sync Http Api
pub type HttpApi = http_api::HttpApi;

/// Async Http Api
pub type HttpApiAsync = http_api_async::HttpApi;

pub struct Account {
    pub address: ImplicitAddress,
    pub private_key: PrivateKey,
    pub public_key: PublicKey,
}

#[inline]
pub const fn base_url() -> &'static str {
    "https://api.tez.ie/rpc/edonet"
}

pub fn build_http_apis() -> (HttpApi, HttpApiAsync) {
    (HttpApi::new(base_url()), HttpApiAsync::new(base_url()))
}

pub fn build_sandbox_http_apis(port: u32) -> (HttpApi, HttpApiAsync) {
    (
        HttpApi::new(format!("http://localhost:{}", port)),
        HttpApiAsync::new(format!("http://localhost:{}", port)),
    )
}

pub fn account_1() -> Account {
    Account {
        address: ImplicitAddress::from_base58check("tz1cY73TfXg3CYxGhQwJviYG8gN7WYn9NM2t").unwrap(),
        private_key: PrivateKey::from_base58check(
            "edsk324owL57dydrDEBHXYSnNq7zAAoVzc61sugi2rD4qcVrMzogG9",
        )
        .unwrap(),
        public_key: PublicKey::from_base58check(
            "edpku7rPeaYLJpyfwkmGJs6cyxUQQEZuKgGyMvLxWfLiAckGT5WrRZ",
        )
        .unwrap(),
    }
}

pub fn sandbox_account_1() -> Account {
    Account {
        address: ImplicitAddress::from_base58check("tz1VSUr8wwNhLAzempoch5d6hLRiTh8Cjcjb").unwrap(),
        private_key: PrivateKey::from_base58check(
            "edsk3QoqBuvdamxouPhin7swCvkQNgq4jP5KZPbwWNnwdZpSpJiEbq",
        )
        .unwrap(),
        public_key: PublicKey::from_base58check(
            "edpkvGfYw3LyB1UcCahKQk4rF2tvbMUk8GFiTuMjL75uGXrpvKXhjn",
        )
        .unwrap(),
    }
}

pub fn sandbox_account_2() -> Account {
    Account {
        address: ImplicitAddress::from_base58check("tz1aSkwEot3L2kmUvcoxzjMomb9mvBNuzFK6").unwrap(),
        private_key: PrivateKey::from_base58check(
            "edsk3RFfvaFaxbHx8BMtEW1rKQcPtDML3LXjNqMNLCzC3wLC1bWbAt",
        )
        .unwrap(),
        public_key: PublicKey::from_base58check(
            "edpkurPsQ8eUApnLUJ9ZPDvu98E8VNj4KtJa1aZr16Cr5ow5VHKnz4",
        )
        .unwrap(),
    }
}
