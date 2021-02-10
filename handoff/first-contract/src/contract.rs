use cosmwasm_std::{
    to_binary, Api, Binary, CosmosMsg, Env, Extern, HandleResponse, HumanAddr, InitResponse,
    Querier, StdError, StdResult, Storage, WasmMsg,
};
use first_export::ExportData;

use crate::msg::{HandleMsg, InitMsg, QueryMsg, SecondContractHandleMsg};
use crate::state::{config, config_read, ContractMode, State};
use cosmwasm_storage::Singleton;

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    _msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State {
        owner: env.message.sender,
        migration_addr: None,
        migration_secret: None,
        mode: ContractMode::Running,
    };

    config(&mut deps.storage).save(&state)?;

    Ok(InitResponse::default())
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
    let conf = config(&mut deps.storage);
    let state = conf.load()?;
    if let ContractMode::Migrated = state.mode {
        return Err(StdError::generic_err(format!(
            "This contract has been migrated to {:?}. No further state changes are allowed!",
            state.migration_addr.unwrap_or_default()
        )));
    }

    match msg {
        HandleMsg::Migrate { address, code_hash } => migrate(env, conf, address, code_hash),
    }
}

pub fn migrate<S: Storage>(
    env: Env,
    mut conf: Singleton<S, State>,
    address: HumanAddr,
    code_hash: String,
) -> StdResult<HandleResponse> {
    let mut state = conf.load()?;
    if env.message.sender != state.owner {
        return Err(StdError::generic_err(
            "Only the admin can set the contract to migrate!",
        ));
    }
    if state.migration_addr.is_some() {
        return Err(StdError::generic_err(
            "The contract has already been migrated!",
        ));
    }

    // Generate the secret in some way
    let secret = Binary::from(b"asdfgh");

    state.migration_addr = Some(address.clone());
    state.mode = ContractMode::Migrated;
    state.migration_secret = Some(secret.clone());
    conf.save(&state)?;

    let messages = vec![CosmosMsg::Wasm(WasmMsg::Execute {
        msg: to_binary(&SecondContractHandleMsg::SetMigrationSecret { secret })?,
        send: vec![],
        contract_addr: address,
        callback_code_hash: code_hash,
    })];
    Ok(HandleResponse {
        log: vec![],
        data: None,
        messages,
    })
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::MigrationAddress {} => to_binary(&query_migration_address(deps)?),
        QueryMsg::ExportedData { secret } => to_binary(&query_exported_data(deps, secret)?),
    }
}

fn query_migration_address<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
) -> StdResult<Option<HumanAddr>> {
    let state = config_read(&deps.storage).load()?;
    Ok(state.migration_addr)
}

/// This function can also be set up to take some sort of paging parameter.
/// Especially for exporting contracts that store large datasets, it's impossible
/// to extract all the information in one go.
fn query_exported_data<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    secret: Binary,
) -> StdResult<ExportData> {
    let state = config_read(&deps.storage).load()?;
    let migration_secret = state
        .migration_secret
        .ok_or_else(|| StdError::generic_err("This contract has not been migrated yet"))?;
    if migration_secret != secret {
        return Err(StdError::generic_err(
            "This contract has not been migrated yet",
        ));
    }

    // Access storage and export the necessary information that would be used by the new contract.
    Ok(ExportData {
        accounts: vec![],
        decimals: 6,
        name: "example".to_string(),
    })
}
