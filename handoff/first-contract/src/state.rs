use serde::{Deserialize, Serialize};

use cosmwasm_std::{Binary, HumanAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static CONFIG_KEY: &[u8] = b"config";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct State {
    pub owner: HumanAddr,
    pub migration_addr: Option<HumanAddr>,
    pub migration_secret: Option<Binary>,
    pub mode: ContractMode,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum ContractMode {
    Running,
    Migrated,
}

pub fn config<S: Storage>(storage: &mut S) -> Singleton<S, State> {
    singleton(storage, CONFIG_KEY)
}

pub fn config_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, State> {
    singleton_read(storage, CONFIG_KEY)
}
