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
    #[prost(uint64, required, tag="1")]
    pub slot: u64,
    #[prost(string, required, tag="2")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="3")]
    pub instruction_index: u32,
    #[prost(string, required, tag="4")]
    pub from: ::prost::alloc::string::String,
    #[prost(string, required, tag="5")]
    pub to: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="6")]
    pub lamports: u64,
    #[prost(uint32, required, tag="7")]
    pub inner_instruction_index: u32,
    #[prost(bool, required, tag="8")]
    pub is_inner_instruction: bool,
    #[prost(uint32, required, tag="9")]
    pub transaction_index: u32,
    #[prost(uint64, required, tag="10")]
    pub tx_fee: u64,
}
// @@protoc_insertion_point(module)
