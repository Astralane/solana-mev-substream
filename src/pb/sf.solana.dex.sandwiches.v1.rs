// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SandwichOutput {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<Sandwich>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sandwich {
    #[prost(message, required, tag="1")]
    pub frontrun: SwapDto,
    #[prost(message, repeated, tag="2")]
    pub victim_swaps: ::prost::alloc::vec::Vec<SwapDto>,
    #[prost(message, required, tag="3")]
    pub backrun: SwapDto,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapDto {
    #[prost(uint64, required, tag="1")]
    pub block_slot: u64,
    #[prost(string, required, tag="2")]
    pub tx_id: ::prost::alloc::string::String,
    #[prost(string, required, tag="3")]
    pub signer: ::prost::alloc::string::String,
    #[prost(string, required, tag="4")]
    pub pool_address: ::prost::alloc::string::String,
    #[prost(string, required, tag="5")]
    pub token_in: ::prost::alloc::string::String,
    #[prost(string, required, tag="6")]
    pub token_out: ::prost::alloc::string::String,
    #[prost(double, required, tag="7")]
    pub amount_in: f64,
    #[prost(double, required, tag="8")]
    pub amount_out: f64,
    #[prost(uint64, required, tag="9")]
    pub txn_fee: u64,
    #[prost(string, required, tag="10")]
    pub multilocation: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
