use std::num::ParseIntError;

use data_encoding::DecodeError;
use thiserror::Error;
pub type Result<T> = core::result::Result<T, AppError>;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("serde_json error: {0}")]
    SerdeError(#[from] serde_json::Error),
    #[error("Gossip error: {0}")]
    GossipApiError(#[from] iroh_gossip::api::ApiError),
    #[error("Anyhow error: {0}")]
    AnyhowError(#[from] anyhow::Error),
    #[error("Decode error: {0}")]
    DecodeError(#[from] DecodeError),
    #[error("Crypt error: {0}")]
    CryptError(#[from] magic_crypt::MagicCryptError),
    #[error("Parse error: {0}")]
    ParseIntError(#[from] ParseIntError),
    #[error("Inquire error: {0}")]
    InquireError(#[from] inquire::InquireError),
    #[error("SeaOrm error: {0}")]
    OrmError(#[from] sea_orm::DbErr),
}
