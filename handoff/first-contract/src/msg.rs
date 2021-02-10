use cosmwasm_std::{Binary, HumanAddr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    /// Set migration secret, and the address of the new contract
    Migrate {
        address: HumanAddr,
        code_hash: String,
    },
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SecondContractHandleMsg {
    SetMigrationSecret { secret: Binary },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// What's the new address?
    MigrationAddress {},
    /// The new contract can query this to extract all the information.
    ExportedData { secret: Binary },
}
