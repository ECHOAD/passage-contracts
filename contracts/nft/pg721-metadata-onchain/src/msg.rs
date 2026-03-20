pub use pg721::msg::*;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct Metadata {
    /// Rich metadata-on-chain is explicitly non-default for metaverse assets.
    ///
    /// Active Passage collections should keep only durable protocol semantics in `extension` and
    /// move runtime/render details behind `token_uri`.
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
    pub nft_type: Option<NftType>,
    pub extension: Option<NftTypeExtension>,
}

pub type Extension = Option<Metadata>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension>;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct MigrateMsg {
    pub minter: String,
}
