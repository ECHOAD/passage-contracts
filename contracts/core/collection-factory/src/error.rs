use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Contract is paused")]
    ContractPaused {},

    #[error("Creator wallet is not approved")]
    CreatorNotApproved {},

    #[error("Creator wallet is not allowed to create collections in the configured ecosystem")]
    CreatorNotApprovedInEcosystem {},

    #[error("Configured ecosystem was not found in registry")]
    EcosystemNotFound {},

    #[error("Collection name cannot be empty")]
    EmptyName {},

    #[error("Collection symbol cannot be empty")]
    EmptySymbol {},

    #[error("Collection minter cannot be empty")]
    EmptyMinter {},

    #[error("Collection description cannot be empty")]
    EmptyDescription {},

    #[error("Collection image cannot be empty")]
    EmptyImage {},

    #[error("Ecosystem id cannot be empty")]
    EmptyEcosystemId {},

    #[error("Collection code id must be greater than zero")]
    InvalidCodeId {},

    #[error("Invalid collection instantiate reply data")]
    InvalidInstantiateReplyData {},

    #[error("Pending creation not found for reply id: {id}")]
    PendingCreationNotFound { id: u64 },
}
