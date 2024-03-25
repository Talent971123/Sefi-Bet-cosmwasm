//! Defines *InstantiateMsg*, *ExecuteMsg* and *QueryMsg*.
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    pub fee_address: Addr,
    pub start_flag: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct UserBetInfo {
    pub user_address: Addr,
    pub user_amount: u128,
    pub user_order: u128,
    pub coin_type: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddBetUserInfinite {},
    AddBetUserWhale {},
    AddBetUserShrimp {},
    ToggleGameInfinte {},
    ToggleGameWhale {},
    ToggleGameShrimp {},
    EndGameInfinte { seed: u64 },
    EndGameWhale { seed: u64 },
    EndGameShrimp { seed: u64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {}
