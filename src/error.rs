use derive_more::Display;
use thiserror::Error;

#[derive(Error, Debug, Display)]
pub enum MevSubstreamError {
    DecodeInstructionError(String),
    IoError(#[from] std::io::Error),
}
