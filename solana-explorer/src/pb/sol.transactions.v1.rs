// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instructions {
    #[prost(message, repeated, tag="1")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
    #[prost(uint64, tag="2")]
    pub slot: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Instruction {
    #[prost(string, tag="1")]
    pub program_id: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub accounts: ::prost::alloc::vec::Vec<AccountMeta>,
    #[prost(string, tag="3")]
    pub data: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountMeta {
    #[prost(string, tag="1")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(bool, tag="2")]
    pub is_writable: bool,
    #[prost(bool, tag="3")]
    pub is_signer: bool,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, repeated, tag="1")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, repeated, tag="2")]
    pub instructions: ::prost::alloc::vec::Vec<Instruction>,
}
// @@protoc_insertion_point(module)
