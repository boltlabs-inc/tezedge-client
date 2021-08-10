use serde::{Deserialize, Deserializer};
use std::fmt::{self, Display};

use crate::api::TransportError;
use crate::BoxFuture;
use types::{
    Address, BlockHash, FromPrefixedBase58CheckError, ImplicitAddress, OperationHash,
    OriginatedAddress, PublicKey,
};

#[derive(thiserror::Error, Debug)]
pub enum BlockGetOperationsError {
    Transport(#[from] TransportError),
    Base58Decode(#[from] FromPrefixedBase58CheckError),
    Unknown(String),
}

impl Display for BlockGetOperationsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "getting head block hash failed! Reason: ")?;
        match self {
            Self::Transport(err) => err.fmt(f),
            Self::Base58Decode(err) => err.fmt(f),
            Self::Unknown(err) => write!(f, "Unknown! {}", err),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockOperationResultOrigination {
    pub originated_contracts: Vec<OriginatedAddress>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockOperationMetadataOrigination {
    pub operation_result: BlockOperationResultOrigination,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockOperationContentReveal {
    pub source: ImplicitAddress,
    pub public_key: PublicKey,
    #[serde(with = "utils::serde_amount")]
    pub fee: u64,
    #[serde(with = "utils::serde_str")]
    pub counter: u64,
    #[serde(with = "utils::serde_str")]
    pub gas_limit: u64,
    #[serde(with = "utils::serde_str")]
    pub storage_limit: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockOperationContentTransaction {
    pub source: ImplicitAddress,
    pub destination: Address,
    #[serde(with = "utils::serde_amount")]
    pub amount: u64,
    #[serde(with = "utils::serde_amount")]
    pub fee: u64,
    #[serde(with = "utils::serde_str")]
    pub counter: u64,
    #[serde(with = "utils::serde_str")]
    pub gas_limit: u64,
    #[serde(with = "utils::serde_str")]
    pub storage_limit: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockOperationContentDelegation {
    pub source: ImplicitAddress,
    pub delegate: Option<ImplicitAddress>,
    #[serde(with = "utils::serde_amount")]
    pub fee: u64,
    #[serde(with = "utils::serde_str")]
    pub counter: u64,
    #[serde(with = "utils::serde_str")]
    pub gas_limit: u64,
    #[serde(with = "utils::serde_str")]
    pub storage_limit: u64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockOperationContentOrigination {
    pub source: ImplicitAddress,
    #[serde(with = "utils::serde_amount")]
    pub balance: u64,
    #[serde(with = "utils::serde_amount")]
    pub fee: u64,
    #[serde(with = "utils::serde_str")]
    pub counter: u64,
    #[serde(with = "utils::serde_str")]
    pub gas_limit: u64,
    #[serde(with = "utils::serde_str")]
    pub storage_limit: u64,
    pub script: Option<serde_json::Value>,
    pub metadata: BlockOperationMetadataOrigination,
}

#[derive(Debug, Clone)]
pub enum BlockOperationContent {
    Reveal(BlockOperationContentReveal),
    Transaction(BlockOperationContentTransaction),
    Delegation(BlockOperationContentDelegation),
    Origination(BlockOperationContentOrigination),
    Other,
}

impl<'de> Deserialize<'de> for BlockOperationContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        #[derive(Deserialize)]
        #[serde(tag = "kind")]
        enum WithKind {
            reveal(BlockOperationContentReveal),
            transaction(BlockOperationContentTransaction),
            delegation(BlockOperationContentDelegation),
            origination(BlockOperationContentOrigination),
            #[serde(other)]
            other,
        }

        Ok(match WithKind::deserialize(deserializer)? {
            WithKind::reveal(op) => Self::Reveal(op),
            WithKind::transaction(op) => Self::Transaction(op),
            WithKind::delegation(op) => Self::Delegation(op),
            WithKind::origination(op) => Self::Origination(op),
            WithKind::other => Self::Other,
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct BlockOperation {
    pub hash: OperationHash,
    pub branch: BlockHash,
    pub contents: Vec<BlockOperationContent>,
}

pub type BlockGetOperationsResult = Result<Vec<BlockOperation>, BlockGetOperationsError>;

pub trait BlockGetOperations {
    /// Get head block's hash.
    fn block_get_operations(&self) -> BlockGetOperationsResult;
}

pub trait BlockGetOperationsAsync {
    /// Get head block's hash.
    fn block_get_operations(
        &self,
        block: &BlockHash,
    ) -> BoxFuture<'static, BlockGetOperationsResult>;
}

pub(crate) fn block_get_operations_url(base_url: &str, block_hash: &str) -> String {
    format!("{}/chains/main/blocks/{}/operations", base_url, block_hash)
}
