// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionInfoStore {
    #[prost(map="string, message", tag="1")]
    pub store: ::std::collections::HashMap<::prost::alloc::string::String, TransactionDetails>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TransactionDetails {
    #[prost(uint64, tag="1")]
    pub slot: u64,
    #[prost(string, tag="2")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub transaction_index: u32,
    #[prost(string, tag="4")]
    pub signer: ::prost::alloc::string::String,
    #[prost(uint64, tag="5")]
    pub tx_fee: u64,
    #[prost(uint64, tag="6")]
    pub priority_fee: u64,
}
// @@protoc_insertion_point(module)
