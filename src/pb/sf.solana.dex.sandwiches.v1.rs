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
    #[prost(message, repeated, tag="1")]
    pub frontrun: ::prost::alloc::vec::Vec<NormalizedSwap>,
    #[prost(message, repeated, tag="2")]
    pub victim_swaps: ::prost::alloc::vec::Vec<NormalizedSwap>,
    #[prost(message, repeated, tag="3")]
    pub backrun: ::prost::alloc::vec::Vec<NormalizedSwap>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SwapsOutput {
    #[prost(message, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<NormalizedSwap>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NormalizedSwap {
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
    pub tx_fee: u64,
    #[prost(string, required, tag="10")]
    pub multi_location: ::prost::alloc::string::String,
    #[prost(uint32, required, tag="11")]
    pub instruction_index: u32,
    #[prost(bool, required, tag="12")]
    pub is_inner_instruction: bool,
    #[prost(uint32, required, tag="13")]
    pub inner_instruction_index: u32,
    #[prost(uint32, required, tag="14")]
    pub transaction_index: u32,
    #[prost(string, required, tag="15")]
    pub inner_program: ::prost::alloc::string::String,
    #[prost(string, required, tag="16")]
    pub outer_program: ::prost::alloc::string::String,
    #[prost(uint64, required, tag="17")]
    pub priority_fee: u64,
    #[prost(string, required, tag="18")]
    pub block_date: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
