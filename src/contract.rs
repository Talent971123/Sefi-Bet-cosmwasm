//! This module implements `instantiate`, `execute` and `query`.
//! These actions are performed using *wasmd*.

// #[cfg(not(feature = "library"))]
use cosmwasm_std::{
    coin, entry_point, Addr, BankMsg, CosmosMsg, DepsMut, Env, MessageInfo, Response,
};

use cw2::set_contract_version;

use crate::msg::UserBetInfo;
use crate::query::query_config;
use crate::state::{State, CONFIG};
use crate::{
    msg::{ExecuteMsg, InstantiateMsg},
    ContractError,
};

// version info for migration info
const CONTRACT_NAME: &str = "SEFI_BET";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialise a new instance of this contract.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Configure the state for storing
    let config = State {
        fee_address: msg.fee_address,
        user_list_infinte: vec![],
        user_list_shrimp: vec![],
        user_list_whale: vec![],
        pot_total_infinte: 0,
        pot_total_shrimp: 0,
        pot_total_whale: 0,
        betting_start_time: 0,
        start_flag_infinte: true,
        start_flag_whale: true,
        start_flag_shrimp: true,
    };
    // Store
    CONFIG.save(deps.storage, &config)?;
    // Return an Ok() response as everything went well
    Ok(Response::new()
        .add_attribute("fee_address", config.fee_address)
        .add_attribute("betting_start_time", config.betting_start_time.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddBetUserInfinite {} => add_bet_user_infinte(deps, env, info),
        ExecuteMsg::AddBetUserWhale {} => add_bet_user_whale(deps, env, info),
        ExecuteMsg::AddBetUserShrimp {} => add_bet_user_shrimp(deps, env, info),
        ExecuteMsg::ToggleGameInfinte {} => toggle_game_infinte(deps, env, info),
        ExecuteMsg::ToggleGameWhale {} => toggle_game_whale(deps, env, info),
        ExecuteMsg::ToggleGameShrimp {} => toggle_game_shrimp(deps, env, info),
        ExecuteMsg::EndGameInfinte { seed } => end_game_infinite(deps, env, info, seed),
        ExecuteMsg::EndGameShrimp { seed } => end_game_shrimp(deps, env, info, seed),
        ExecuteMsg::EndGameWhale { seed } => end_game_whale(deps, env, info, seed),
    }
}

fn add_bet_user_infinte(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = query_config(deps.as_ref())?;

    //missing the game start or not.
    if config.start_flag_infinte == false {
        return Err(ContractError::CustomError {
            val: String::from("Bet is already started!"),
        });
    }
    //add pot total amount

    let mut funds_amount = 0;
    let mut coin_type: String = String::new();

    for element in info.funds.iter() {
        funds_amount = element.amount.u128();
        coin_type = element.denom.clone();
    }

    if funds_amount == 0 {
        return Err(ContractError::CustomError {
            val: String::from("Not enough funds"),
        });
    }

    let transfer_amount_infinte = BankMsg::Send {
        to_address: env.contract.address.to_string(),
        amount: vec![coin(funds_amount as u128, coin_type.clone())],
    };

    config.pot_total_infinte = config.pot_total_infinte + funds_amount;
    let order_id: usize = config.user_list_infinte.len();
    config.user_list_infinte.push(UserBetInfo {
        user_address: info.sender.clone(),
        user_amount: funds_amount,
        user_order: order_id as u128 + 1,
        coin_type: coin_type,
    });
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_message(CosmosMsg::Bank(transfer_amount_infinte))
        .add_attribute("action", "bet_infinte")
        .add_attribute("bet_user_address", info.sender.to_string())
        .add_attribute("bet_user_funds", funds_amount.to_string()))
}

fn add_bet_user_whale(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = query_config(deps.as_ref())?;

    //missing the game start or not.
    if config.start_flag_whale == false {
        return Err(ContractError::CustomError {
            val: String::from("Bet is already started!"),
        });
    }
    //add pot total amountf

    let mut funds_amount = 0;
    let mut coin_type: String = String::new();

    for element in info.funds.iter() {
        funds_amount = element.amount.u128();
        coin_type = element.denom.clone();
    }

    let transfer_amount_whale = BankMsg::Send {
        to_address: _env.contract.address.to_string(),
        amount: vec![coin(funds_amount as u128, coin_type.clone())],
    };
    //check the balance

    if funds_amount > 500 {
        return Err(ContractError::CustomError {
            val: String::from("Bet amounts are less than 500 SEI"),
        });
    }

    config.pot_total_whale = config.pot_total_whale + funds_amount;
    let order_id: usize = config.user_list_whale.len();
    config.user_list_whale.push(UserBetInfo {
        user_address: info.sender.clone(),
        user_amount: funds_amount,
        user_order: order_id as u128 + 1,
        coin_type: coin_type,
    });
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_message(CosmosMsg::Bank(transfer_amount_whale))
        .add_attribute("action", "bet_whale")
        .add_attribute("bet_user_address", info.sender.to_string())
        .add_attribute("bet_user_funds", funds_amount.to_string()))
}

fn add_bet_user_shrimp(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = query_config(deps.as_ref())?;

    //missing the game start or not.
    if config.start_flag_shrimp == false {
        return Err(ContractError::CustomError {
            val: String::from("Bet is already started!"),
        });
    }
    //add pot total amountf

    let mut funds_amount = 0;
    let mut coin_type: String = String::new();

    for element in info.funds.iter() {
        funds_amount = element.amount.u128();
        coin_type = element.denom.clone();
    }

    if funds_amount > 50 {
        return Err(ContractError::CustomError {
            val: String::from("Bet amounts are less than 50 SEI"),
        });
    }

    let transfer_amount_shrimp = BankMsg::Send {
        to_address: _env.contract.address.to_string(),
        amount: vec![coin(funds_amount as u128, coin_type.clone())],
    };

    config.pot_total_shrimp = config.pot_total_shrimp + funds_amount;
    let order_id: usize = config.user_list_shrimp.len();
    config.user_list_shrimp.push(UserBetInfo {
        user_address: info.sender.clone(),
        user_amount: funds_amount,
        user_order: order_id as u128 + 1,
        coin_type: coin_type,
    });
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_message(CosmosMsg::Bank(transfer_amount_shrimp))
        .add_attribute("action", "bet_shrimp")
        .add_attribute("bet_user_address", info.sender.to_string())
        .add_attribute("bet_user_funds", funds_amount.to_string()))
}

fn toggle_game_infinte(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = query_config(deps.as_ref())?;
    if info.sender != config.fee_address {
        return Err(ContractError::CustomError {
            val: String::from("Authorization Error"),
        });
    }
    config.start_flag_infinte = !config.start_flag_infinte;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "bet")
        .add_attribute("bet_user_address", info.sender.to_string()))
}

fn toggle_game_whale(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = query_config(deps.as_ref())?;
    if info.sender != config.fee_address {
        return Err(ContractError::CustomError {
            val: String::from("Authorization Error"),
        });
    }
    config.start_flag_whale = !config.start_flag_whale;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "bet")
        .add_attribute("bet_user_address", info.sender.to_string()))
}

fn toggle_game_shrimp(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    let mut config = query_config(deps.as_ref())?;
    if info.sender != config.fee_address {
        return Err(ContractError::CustomError {
            val: String::from("Authorization Error"),
        });
    }
    config.start_flag_shrimp = !config.start_flag_shrimp;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "bet")
        .add_attribute("bet_user_address", info.sender.to_string()))
}
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        let start_seed = seed;
        Lcg { state: start_seed }
    }

    fn next(&mut self) -> u64 {
        // Parameters for Numerical Recipes LCG
        let a: u64 = 1664525;
        let c: u64 = 1013904223;
        let m: u64 = 2u64.pow(32);
        self.state = (a.wrapping_mul(self.state).wrapping_add(c)) % m;
        self.state
    }
}

pub fn generate_random_value(seed: u64) -> u128 {
    let mut lcg = Lcg::new(seed);

    let timestamp = lcg.next(); // Get the current block's timestamp
    let random_value = timestamp % 10000; // Generate a random number between 1 and 100
    println!("randome value {}", random_value);
    random_value as u128
}

fn end_game_infinite(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    seed: u64,
) -> Result<Response, ContractError> {
    let mut config = query_config(deps.as_ref())?;

    if _info.sender != config.fee_address {
        return Err(ContractError::CustomError {
            val: String::from("Authorization Error"),
        });
    }
    let infinte_random_number = generate_random_value(seed);

    if config.start_flag_infinte == true {
        return Err(ContractError::CustomError {
            val: String::from("Game is still open"),
        });
    }
    //for infinte
    let mut previous_percentage_infinte = 0;
    let mut winner_address_infinte: Addr = Addr::unchecked("");
    let transfer_fees_infinte: BankMsg;
    let transfer_prize_infinte: BankMsg;

    let mut coin_type: String = String::from("usei");

    if config.pot_total_infinte == 0 {
        return Err(ContractError::CustomError {
            val: String::from("pot total infinte is zero so can't end game"),
        });
    }

    for infinite_user in &config.user_list_infinte {
        coin_type = infinite_user.coin_type.clone();
        let percentage: u128 = previous_percentage_infinte
            + infinite_user.user_amount * 100 * 100 / config.pot_total_infinte;

        if previous_percentage_infinte < infinte_random_number && percentage > infinte_random_number
        {
            winner_address_infinte = infinite_user.user_address.clone();
            println!("winner Address, {}", winner_address_infinte.to_string());
            break;
        }

        previous_percentage_infinte = percentage;
    }

    // for infinte
    let fees = config.pot_total_infinte * 4 / 100;

    let prize = config.pot_total_infinte - fees;

    transfer_fees_infinte = BankMsg::Send {
        to_address: config.fee_address.to_string(),
        amount: vec![coin(fees as u128, coin_type.clone())],
    };

    transfer_prize_infinte = BankMsg::Send {
        to_address: winner_address_infinte.to_string(),
        amount: vec![coin(prize as u128, coin_type.clone())],
    };

    config.user_list_infinte = vec![];
    config.pot_total_infinte = 0;
    config.start_flag_infinte = !config.start_flag_infinte;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(transfer_fees_infinte))
        .add_message(CosmosMsg::Bank(transfer_prize_infinte))
        .add_attribute("action", "end_game")
        .add_attribute("winner_infinte", winner_address_infinte.to_string())
        .add_attribute("method", "reset_for_next_round"))
}

fn end_game_whale(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    seed: u64,
) -> Result<Response, ContractError> {
    let mut config = query_config(deps.as_ref())?;
    if _info.sender != config.fee_address {
        return Err(ContractError::CustomError {
            val: String::from("Authorization Error"),
        });
    }
    let whale_random_number = generate_random_value(seed);

    //for whale
    let mut previous_percentage_whale = 0;
    let mut winner_address_whale: Addr = Addr::unchecked("");
    let transfer_fees_whale: BankMsg;
    let transfer_prize_whale: BankMsg;
    let mut coin_type: String = String::from("usei");

    if config.start_flag_whale == true {
        return Err(ContractError::CustomError {
            val: String::from("Game is still open"),
        });
    }

    if config.pot_total_whale == 0 {
        return Err(ContractError::CustomError {
            val: String::from("pot total whale is zero so can't end game"),
        });
    }

    //for whale
    for whale_user in &config.user_list_whale {
        let percentage = previous_percentage_whale
            + whale_user.user_amount * 100 * 100 / config.pot_total_infinte;
        coin_type = whale_user.coin_type.clone();
        if previous_percentage_whale < whale_random_number && percentage > whale_random_number {
            winner_address_whale = whale_user.user_address.clone();
            break;
        }

        previous_percentage_whale = percentage;
    }
    // for wale send
    let fees = config.pot_total_infinte * 4 / 100;

    let prize = config.pot_total_infinte - fees;

    transfer_fees_whale = BankMsg::Send {
        to_address: config.fee_address.to_string(),
        amount: vec![coin(fees as u128, coin_type.clone())],
    };

    transfer_prize_whale = BankMsg::Send {
        to_address: winner_address_whale.to_string(),
        amount: vec![coin(prize as u128, coin_type.clone())],
    };

    config.user_list_infinte = vec![];
    config.pot_total_infinte = 0;
    config.start_flag_infinte = !config.start_flag_infinte;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(transfer_fees_whale))
        .add_message(CosmosMsg::Bank(transfer_prize_whale))
        .add_attribute("action", "end_game")
        .add_attribute("winner_infinte", winner_address_whale.to_string())
        .add_attribute("method", "reset_for_next_round"))
}

fn end_game_shrimp(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    seed: u64,
) -> Result<Response, ContractError> {
    let mut config = query_config(deps.as_ref())?;
    if _info.sender != config.fee_address {
        return Err(ContractError::CustomError {
            val: String::from("Authorization Error"),
        });
    }
    let shrimp_random_number = generate_random_value(seed);

    //for shrimp
    let mut previous_percentage_shrimp = 0;
    let mut winner_address_shrimp: Addr = Addr::unchecked("");
    let transfer_fees_shrimp: BankMsg;
    let transfer_prize_shrimp: BankMsg;
    let mut coin_type: String = String::from("usei");
    if config.start_flag_shrimp == true {
        return Err(ContractError::CustomError {
            val: String::from("Game is still open"),
        });
    }

    if config.pot_total_shrimp == 0 {
        return Err(ContractError::CustomError {
            val: String::from("pot total infinte is zero so can't end game"),
        });
    }

    //for shrimp
    for shrimp_user in &config.user_list_shrimp {
        let percentage = previous_percentage_shrimp
            + shrimp_user.user_amount * 100 * 100 / config.pot_total_infinte;
        coin_type = shrimp_user.coin_type.clone();
        if previous_percentage_shrimp < shrimp_random_number && percentage > shrimp_random_number {
            winner_address_shrimp = shrimp_user.user_address.clone();
            print!("winner address {}", winner_address_shrimp.to_string());
            break;
        }

        previous_percentage_shrimp = percentage;
    }

    // for wale send
    let fees = config.pot_total_infinte * 4 / 100;

    let prize = config.pot_total_infinte - fees;

    transfer_fees_shrimp = BankMsg::Send {
        to_address: config.fee_address.to_string(),
        amount: vec![coin(fees as u128, coin_type.clone())],
    };

    transfer_prize_shrimp = BankMsg::Send {
        to_address: winner_address_shrimp.to_string(),
        amount: vec![coin(prize as u128, coin_type.clone())],
    };

    config.user_list_infinte = vec![];
    config.pot_total_infinte = 0;
    config.start_flag_infinte = !config.start_flag_infinte;

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_message(CosmosMsg::Bank(transfer_fees_shrimp))
        .add_message(CosmosMsg::Bank(transfer_prize_shrimp))
        .add_attribute("action", "end_game")
        .add_attribute("winner_infinte", winner_address_shrimp.to_string())
        .add_attribute("method", "reset_for_next_round"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, Addr};

    const DENOM: &str = "sei";

    fn init_msg(fee_address: Addr, start_flag: bool) -> InstantiateMsg {
        InstantiateMsg {
            fee_address,
            start_flag,
        }
    }

    #[test]
    fn proper_initialization() {
        // Create mock dependencies and environment
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &coins(0, &DENOM.to_string()));

        // Successful instantiation
        let msg = init_msg(Addr::unchecked("fee_address"), true);
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let stored_state = query_config(deps.as_ref()).unwrap();
        assert_eq!(stored_state.fee_address.to_string(), "fee_address");
        assert_eq!(stored_state.user_list_infinte, vec![]);
        assert_eq!(stored_state.user_list_whale, vec![]);
        assert_eq!(stored_state.user_list_shrimp, vec![]);
        assert_eq!(stored_state.pot_total_infinte, 0);
        assert_eq!(stored_state.pot_total_shrimp, 0);
        assert_eq!(stored_state.pot_total_whale, 0);
    }
    #[test]
    fn test_add_bet_user_infinte() {
        let mut deps = mock_dependencies();
        let info = mock_info("first_bet_user", &coins(10, &DENOM.to_string()));
        let msg = init_msg(Addr::unchecked("fee_address"), true);
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let res = add_bet_user_infinte(deps.as_mut(), mock_env(), info).unwrap();

        //check message len
        assert_eq!(3, res.attributes.len());
        assert_eq!(1, res.messages.len());

        let stored_state = query_config(deps.as_ref()).unwrap();

        let infite_user = stored_state.user_list_infinte.first().unwrap();

        assert_eq!("first_bet_user", infite_user.user_address.to_string());
        assert_eq!(10, infite_user.user_amount);

        assert_eq!(10, stored_state.pot_total_infinte);
    }

    #[test]
    fn test_add_bet_user_whale() {
        let mut deps = mock_dependencies();
        let info = mock_info("first_bet_user", &coins(10, &DENOM.to_string()));
        let msg = init_msg(Addr::unchecked("fee_address"), true);
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let res = add_bet_user_whale(deps.as_mut(), mock_env(), info).unwrap();

        //check message len
        assert_eq!(3, res.attributes.len());
        assert_eq!(1, res.messages.len());

        let stored_state = query_config(deps.as_ref()).unwrap();

        let infite_user = stored_state.user_list_whale.first().unwrap();

        assert_eq!("first_bet_user", infite_user.user_address.to_string());
        assert_eq!(10, infite_user.user_amount);

        assert_eq!(10, stored_state.pot_total_whale);
    }

    #[test]
    fn test_add_bet_user_shrimp() {
        let mut deps = mock_dependencies();
        let info = mock_info("first_bet_user", &coins(10, &DENOM.to_string()));
        let msg = init_msg(Addr::unchecked("fee_address"), true);
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let res = add_bet_user_shrimp(deps.as_mut(), mock_env(), info).unwrap();

        //check message len
        assert_eq!(3, res.attributes.len());
        assert_eq!(1, res.messages.len());

        let stored_state = query_config(deps.as_ref()).unwrap();

        let infite_user = stored_state.user_list_shrimp.first().unwrap();

        assert_eq!("first_bet_user", infite_user.user_address.to_string());
        assert_eq!(10, infite_user.user_amount);

        assert_eq!(10, stored_state.pot_total_shrimp);
    }

    #[test]
    fn test_end_game() {
        let mut deps = mock_dependencies();
        let info = mock_info("fee_address", &coins(10, &DENOM.to_string()));
        let msg = init_msg(Addr::unchecked("fee_address"), true);
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let first_bet_user = mock_info("first_bet_user", &coins(100, &DENOM.to_string()));
        let _res: Response =
            add_bet_user_infinte(deps.as_mut(), mock_env(), first_bet_user.clone()).unwrap();

        let second_bet_user = mock_info("second_bet_user", &coins(50, &DENOM.to_string()));
        let _res =
            add_bet_user_infinte(deps.as_mut(), mock_env(), second_bet_user.clone()).unwrap();

        let last_bet_user = mock_info("last_bet_user", &coins(50, &DENOM.to_string()));
        let _res = add_bet_user_infinte(deps.as_mut(), mock_env(), last_bet_user.clone()).unwrap();
        let res = end_game_infinite(deps.as_mut(), mock_env(), info, 1231231809300).unwrap();

        //check message len
        assert_eq!(3, res.attributes.len());
        assert_eq!(2, res.messages.len());
    }
}
