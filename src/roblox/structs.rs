use serde::{Deserialize, Serialize};
use serde_repr::{Serialize_repr, Deserialize_repr};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum AssetType {
    #[default]
    Image = 1,
    Audio = 3,
    Mesh = 4,
    Lua = 5,
    Model = 10,
    Decal = 13
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize)]
pub enum CreatorType {
    #[default]
    User,
    Group,
}

#[derive(
Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Default, Serialize, Deserialize,
)]
pub struct Creator {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "HasVerifiedBadge")]
    pub has_verified_badge: bool,
    #[serde(rename = "CreatorType")]
    pub creator_type: CreatorType,
    #[serde(rename = "CreatorTargetId")]
    pub target_id: i64,
    #[serde(rename = "Name")]
    pub name: String,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ItemDetails {
    #[serde(rename = "AssetId")]
    pub id: i64,
    #[serde(rename = "TargetId")]
    pub target_id: i64,
    #[serde(rename = "ProductId")]
    pub product_id: i64,
    #[serde(rename = "AssetTypeId")]
    pub asset_type_id: Option<AssetType>,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Description")]
    pub description: String,
    #[serde(rename = "Creator")]
    pub creator: Creator,
    #[serde(rename = "PriceInRobux")]
    pub price_in_robux: Option<u64>,
    #[serde(rename = "IsForSale")]
    pub is_for_sale: Option<bool>,
    #[serde(rename = "IsPublicDomain")]
    pub is_public_domain: Option<bool>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetPurchaseReq {
    #[serde(rename = "expectedCurrency")]
    pub expected_currency: u64,
    #[serde(rename = "expectedPrice")]
    pub expected_price: u64
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetDeliveryLocation {
    #[serde(rename = "assetFormat")]
    pub asset_format: String,
    pub location: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AssetDeliveryResponse {
    pub locations: Vec<AssetDeliveryLocation>,
    #[serde(rename = "requestId")]
    pub request_id: String,
    #[serde(rename = "assetTypeId")]
    pub asset_type_id: AssetType,
}

// {"errors":[{"code":0,"message":"User is not authorized to access Asset."}]}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RobloxError {
    pub code: u16,
    pub message: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RobloxApiError {
    pub errors: Vec<RobloxError>
}