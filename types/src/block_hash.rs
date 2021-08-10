use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::convert::TryInto;
use std::fmt::{self, Debug};

use super::FromPrefixedBase58CheckError;
use crypto::base58check::{FromBase58Check, ToBase58Check};
use crypto::{Prefix, WithPrefix, WithoutPrefix};

type BlockHashInner = [u8; 32];

#[derive(Eq, PartialEq, Clone)]
pub struct BlockHash(BlockHashInner);

impl BlockHash {
    /// Parse base58check.
    ///
    /// # Example
    /// ```rust
    /// # use types::BlockHash;
    /// BlockHash::from_base58check("BKrTZWWSV7j5J31WiE1GB7LU8Mn12qtdBQ6z3UGRHhaYBU7jLAr").unwrap();
    /// ```
    pub fn from_base58check(encoded: &str) -> Result<Self, FromPrefixedBase58CheckError> {
        let key_bytes: BlockHashInner = encoded
            .from_base58check()?
            .without_prefix(Prefix::B)?
            .try_into()
            .or(Err(FromPrefixedBase58CheckError::InvalidSize))?;

        Ok(Self(key_bytes))
    }
}

impl ToBase58Check for BlockHash {
    fn to_base58check(&self) -> String {
        self.0.with_prefix(Prefix::B).to_base58check()
    }
}

impl AsRef<BlockHashInner> for BlockHash {
    fn as_ref(&self) -> &BlockHashInner {
        &self.0
    }
}

impl Debug for BlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BlockHash(\"{}\")", self.to_base58check())
    }
}

impl Serialize for BlockHash {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_base58check())
    }
}

impl<'de> Deserialize<'de> for BlockHash {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let encoded = String::deserialize(deserializer)?;

        Self::from_base58check(&encoded).map_err(|err| serde::de::Error::custom(err))
    }
}
