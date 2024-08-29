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
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(string, tag="2")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub instruction_index: u32,
    #[prost(uint32, tag="4")]
    pub inner_instruction_index: u32,
    #[prost(bool, tag="5")]
    pub is_inner_instruction: bool,
    #[prost(string, tag="6")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub to: ::prost::alloc::string::String,
    #[prost(uint64, tag="8")]
    pub lamports: u64,
}
// @@protoc_insertion_point(module)
