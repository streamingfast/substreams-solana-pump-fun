// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Data {
    #[prost(message, repeated, tag="1")]
    pub create_event_list: ::prost::alloc::vec::Vec<CreateEvent>,
    #[prost(message, repeated, tag="2")]
    pub trade_event_list: ::prost::alloc::vec::Vec<TradeEvent>,
    #[prost(message, repeated, tag="3")]
    pub complete_event_list: ::prost::alloc::vec::Vec<CompleteEvent>,
    #[prost(message, repeated, tag="4")]
    pub set_params_event_list: ::prost::alloc::vec::Vec<SetParamsEvent>,
    #[prost(message, repeated, tag="5")]
    pub set_params_list: ::prost::alloc::vec::Vec<SetParams>,
    #[prost(message, repeated, tag="6")]
    pub create_list: ::prost::alloc::vec::Vec<Create>,
    #[prost(message, repeated, tag="7")]
    pub buy_list: ::prost::alloc::vec::Vec<Buy>,
    #[prost(message, repeated, tag="8")]
    pub sell_list: ::prost::alloc::vec::Vec<Sell>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateEvent {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub bonding_curve: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub user: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeEvent {
    #[prost(string, tag="1")]
    pub mint: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub sol_amount: u64,
    #[prost(uint64, tag="3")]
    pub token_amount: u64,
    #[prost(bool, tag="4")]
    pub is_buy: bool,
    #[prost(string, tag="5")]
    pub user: ::prost::alloc::string::String,
    #[prost(int64, tag="6")]
    pub timestamp: i64,
    #[prost(uint64, tag="7")]
    pub virtual_sol_reserves: u64,
    #[prost(uint64, tag="8")]
    pub virtual_token_reserves: u64,
    #[prost(uint64, tag="9")]
    pub real_sol_reserves: u64,
    #[prost(uint64, tag="10")]
    pub real_token_reserves: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompleteEvent {
    #[prost(string, tag="1")]
    pub user: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub mint: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub bonding_curve: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub timestamp: i64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParamsEvent {
    #[prost(string, tag="1")]
    pub fee_recipient: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub initial_virtual_token_reserves: u64,
    #[prost(uint64, tag="3")]
    pub initial_virtual_sol_reserves: u64,
    #[prost(uint64, tag="4")]
    pub initial_real_token_reserves: u64,
    #[prost(uint64, tag="5")]
    pub token_total_supply: u64,
    #[prost(uint64, tag="6")]
    pub fee_basis_points: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetParams {
    #[prost(string, tag="1")]
    pub fee_recipient: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub initial_virtual_token_reserves: u64,
    #[prost(uint64, tag="3")]
    pub initial_virtual_sol_reserves: u64,
    #[prost(uint64, tag="4")]
    pub initial_real_token_reserves: u64,
    #[prost(uint64, tag="5")]
    pub token_total_supply: u64,
    #[prost(uint64, tag="6")]
    pub fee_basis_points: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Create {
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub uri: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Buy {
    #[prost(uint64, tag="1")]
    pub amount: u64,
    #[prost(uint64, tag="2")]
    pub max_sol_cost: u64,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Sell {
    #[prost(uint64, tag="1")]
    pub amount: u64,
    #[prost(uint64, tag="2")]
    pub min_sol_output: u64,
}
// @@protoc_insertion_point(module)
