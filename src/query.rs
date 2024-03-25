use cosmwasm_std::{entry_point, Binary, Deps, Env, StdError, StdResult};

use crate::msg::QueryMsg;
use crate::state::{State, CONFIG};

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {}
}

pub fn query_config(deps: Deps) -> StdResult<State> {
    let res = CONFIG.may_load(deps.storage)?;
    match res {
        Some(val) => Ok(val),
        None => Err(StdError::GenericErr {
            msg: String::from("Unable to load internal state"),
        }),
    }
}
