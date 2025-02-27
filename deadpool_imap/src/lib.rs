pub mod connection;
pub mod session_wrapper;

use anyhow::Result;
use async_native_tls::TlsConnector;
use connection::ConnectionConfig;
use deadpool::{async_trait, managed};
use session_wrapper::SessionWrapper;

type RecycleResult = deadpool::managed::RecycleResult<anyhow::Error>;
// type RecycleError = deadpool::managed::RecycleError<anyhow::Error>;

pub struct ImapConnectionManager {
    config: ConnectionConfig,
}

impl ImapConnectionManager {
    pub fn new(config: ConnectionConfig) -> Self {
        Self { config }
    }
}

#[async_trait]
impl managed::Manager for ImapConnectionManager {
    type Type = SessionWrapper;
    type Error = anyhow::Error;

    async fn create(&self) -> Result<SessionWrapper> {
        // TODO: This is only for testing purposes, REMOVE
        let tls = TlsConnector::new().danger_accept_invalid_certs(true);
        connection::connect(&self.config, tls).await
    }

    async fn recycle(&self, session: &mut SessionWrapper) -> RecycleResult {
        match session.clear().await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Client error: {0}")]
    ClientError(#[from] session_wrapper::ImapError),
}
