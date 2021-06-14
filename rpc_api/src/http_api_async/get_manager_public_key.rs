use types::{Address, PublicKey};
use crate::BoxFuture;
use crate::api::{
    get_manager_key_url,
    GetManagerPublicKeyAsync, GetManagerPublicKeyResult,
    TransportError, GetManagerPublicKeyError, GetManagerPublicKeyErrorKind,
};
use crate::http_api_async::HttpApi;

impl From<reqwest::Error> for GetManagerPublicKeyErrorKind {
    fn from(error: reqwest::Error) -> Self {
        if let Some(status) = error.status() {
            Self::Unknown(format!(
                "Http status: ({}) {}",
                status,
                error,
            ))
        } else {
            Self::Transport(TransportError(Box::new(error)))
        }
    }
}

impl GetManagerPublicKeyAsync for HttpApi {
    fn get_manager_public_key(
        &self,
        addr: &Address,
    ) -> BoxFuture<'static, GetManagerPublicKeyResult> {
        let req = self.client.get(&get_manager_key_url(&self.base_url, addr));
        let addr = addr.clone();
        Box::pin(async move {
            Ok(req
                .send().await
                .map_err(|err| GetManagerPublicKeyError::new(&addr, err))?
                .json::<Option<String>>().await
                .map_err(|err| GetManagerPublicKeyError::new(&addr, err))?
                .map(|key| PublicKey::from_base58check(&key))
                .transpose()
                .map_err(|err| GetManagerPublicKeyError::new(&addr, err))?)
        })
    }
}
