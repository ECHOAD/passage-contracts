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

    #[error("Ecosystem already exists: {id}")]
    EcosystemAlreadyExists { id: String },

    #[error("Ecosystem not found: {id}")]
    EcosystemNotFound { id: String },

    #[error("Collection already registered: {address}")]
    CollectionAlreadyRegistered { address: String },

    #[error("Collection not found: {address}")]
    CollectionNotFound { address: String },

    #[error("Invalid collection contract: {address}")]
    InvalidCollectionContract { address: String },

    #[error("Minter already authorized: {address}")]
    MinterAlreadyAuthorized { address: String },

    #[error("Minter not authorized: {address}")]
    MinterNotAuthorized { address: String },

    #[error("Invalid ID format: {id}")]
    InvalidIdFormat { id: String },

    #[error("Name cannot be empty")]
    EmptyName {},

    #[error("Detail cannot be empty")]
    EmptyDescription {},

    #[error("At least one image is required")]
    EmptyImages {},

    #[error("Only ecosystem admin can perform this action")]
    NotEcosystemAdmin {},

    #[error("Creator is not approved to register ecosystems")]
    EcosystemCreatorNotApproved {},

    #[error("Address is not approved as member for ecosystem: {ecosystem_id}")]
    EcosystemMemberNotApproved { ecosystem_id: String },

    #[error("Only collection creator can perform this action")]
    NotCollectionCreator {},
}
