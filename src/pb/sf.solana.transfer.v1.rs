// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransferOutput {
    #[prost(message, repeated, tag="1")]
    pub transfers: ::prost::alloc::vec::Vec<SystemTransfer>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SystemTransfer {
    #[prost(string, tag="1")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub to: ::prost::alloc::string::String,
    #[prost(uint64, tag="3")]
    pub lamports: u64,
}
// @@protoc_insertion_point(module)
