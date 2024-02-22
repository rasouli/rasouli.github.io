use thiserror::Error;
use tokio::sync::mpsc::error::SendError;

use crate::models::IpPortScanResult;

#[derive(Error, Debug)]
pub enum AppErrors {
    #[error("A Tokio Runtime must be provided to setup the App")]
    NoRuntimeProvidedError,
    #[error("Unable to send scan result {result:?} over tokio channel {channel}")]
    IpScanResultChannelSendError {
        channel: String,
        result: IpPortScanResult,
        source: SendError<IpPortScanResult>,
    },
}
