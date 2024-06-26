use fuel_core_storage::{
    Error as StorageError,
    IsNotFound,
};
use std::net::SocketAddr;

pub mod api_service;
pub mod database;
pub(crate) mod metrics_extension;
pub mod ports;
pub mod storage;
pub(crate) mod view_extension;
pub mod worker_service;

#[derive(Clone, Debug)]
pub struct Config {
    pub addr: SocketAddr,
    pub utxo_validation: bool,
    pub debug: bool,
    pub vm_backtrace: bool,
    pub max_tx: usize,
    pub max_depth: usize,
    pub chain_name: String,
}

pub trait IntoApiResult<T> {
    fn into_api_result<NewT, E>(self) -> Result<Option<NewT>, E>
    where
        NewT: From<T>,
        E: From<StorageError>;
}

impl<T> IntoApiResult<T> for Result<T, StorageError> {
    fn into_api_result<NewT, E>(self) -> Result<Option<NewT>, E>
    where
        NewT: From<T>,
        E: From<StorageError>,
    {
        if self.is_not_found() {
            Ok(None)
        } else {
            Ok(Some(self?.into()))
        }
    }
}
