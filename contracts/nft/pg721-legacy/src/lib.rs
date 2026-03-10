use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Empty;
pub use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse, QueryMsg};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub name: String,
    pub base: String,
    pub accessory: Vec<String>,
    pub background: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Trait>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

pub type Extension = Option<Metadata>;

pub type Cw721PassageContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension>;

#[cfg(not(feature = "library"))]
pub mod entry {

    use crate::{Cw721PassageContract, ExecuteMsg};
    use cosmwasm_std::{entry_point, StdError};
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw2::set_contract_version;
    use cw721_base::{ContractError, InstantiateMsg, QueryMsg};

    // This is a simple type to let us handle empty extensions

    const CONTRACT_NAME: &str = "crates.io:cw721-passage";
    const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        let res = Cw721PassageContract::default().instantiate(deps.branch(), env, info, msg)?;
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        if let ExecuteMsg::Burn { .. } = msg {
            return Err(ContractError::Std(StdError::generic_err(
                "Operation not allowed",
            )));
        }
        Cw721PassageContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721PassageContract::default().query(deps, env, msg)
    }
}

#[cfg(test)]
mod tests;
