use cosmwasm_std::{Coin, HumanAddr};
use serde::{Deserialize, Serialize};

/// Some data structure that you want to export from the contract upon migration.
#[derive(Serialize, Deserialize)]
pub struct ExportData {
    pub name: String,
    pub decimals: u8,
    pub accounts: Vec<Account>,
}

#[derive(Serialize, Deserialize)]
pub struct Account {
    pub addr: HumanAddr,
    pub funds: Vec<Coin>,
}
