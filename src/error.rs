use derive_more::Display;
use thiserror::Error;

#[derive(Error, Debug, Display)]
pub enum MevSubstreamError {
    DecodeInstructionError(u32),
    IoError(#[from] std::io::Error),
}
