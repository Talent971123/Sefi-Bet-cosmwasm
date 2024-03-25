//! Defines the state and tokeninfo structs

use crate::msg::UserBetInfo;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub fee_address: Addr,
    pub user_list_infinte: Vec<UserBetInfo>,
    pub user_list_whale: Vec<UserBetInfo>,
    pub user_list_shrimp: Vec<UserBetInfo>,

    pub pot_total_infinte: u128,
    pub pot_total_whale: u128,
    pub pot_total_shrimp: u128,
    pub betting_start_time: u128,

    pub start_flag_infinte: bool,
    pub start_flag_whale: bool,
    pub start_flag_shrimp: bool,
}

pub const CONFIG: Item<State> = Item::new("config");
