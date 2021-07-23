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
        address: ImplicitAddress::from_base58check("tz1KqTpEZ7Yob7QbPE4Hy4Wo8fHG8LhKxZSx").unwrap(),
        private_key: PrivateKey::from_base58check(
            "edsk3gUfUPyBSfrS9CCgmCiQsTCHGkviBDusMxDJstFtojtc1zcpsh",
        )
        .unwrap(),
        public_key: PublicKey::from_base58check(
            "edpkuBknW28nW72KG6RoHtYW7p12T6GKc7nAbwYX5m8Wd9sDVC9yav",
        )
        .unwrap(),
    }
}

pub fn sandbox_account_2() -> Account {
    Account {
        address: ImplicitAddress::from_base58check("tz1gjaF81ZRRvdzjobyfVNsAeSC6PScjfQwN").unwrap(),
        private_key: PrivateKey::from_base58check(
            "edsk39qAm1fiMjgmPkw1EgQYkMzkJezLNewd7PLNHTkr6w9XA2zdfo",
        )
        .unwrap(),
        public_key: PublicKey::from_base58check(
            "edpktzNbDAUjUk697W7gYg2CRuBQjyPxbEg8dLccYYwKSKvkPvjtV9",
        )
        .unwrap(),
    }
}
