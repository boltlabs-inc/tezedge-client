use std::fmt::{self, Display};

use crate::api::TransportError;
use crate::BoxFuture;

#[derive(thiserror::Error, Debug)]
pub enum GetChainIDError {
    Transport(#[from] TransportError),
    Unknown(String),
}

impl Display for GetChainIDError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "getting main chain id failed! Reason: ")?;
        match self {
            Self::Transport(err) => err.fmt(f),
            Self::Unknown(err) => write!(f, "Unknown! {}", err),
        }
    }
}

pub type GetChainIDResult = Result<String, GetChainIDError>;

pub trait GetChainID {
    fn get_chain_id(&self) -> GetChainIDResult;
}

pub trait GetChainIDAsync {
    fn get_chain_id(&self) -> BoxFuture<'static, GetChainIDResult>;
}

pub(crate) fn get_chain_id_url(base_url: &str) -> String {
    format!("{}/chains/main/chain_id", base_url)
}
