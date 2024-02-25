use crate::Backend;  

mod structs;
mod rbxm;

impl Backend {
    pub async fn whitelist_asset(&self, asset_id: u64, user_id_requesting: u64) -> Result<(), Box<dyn std::error::Error>> {
        if !self.user_own_asset_internal(user_id_requesting, asset_id).await? {
            return Err("User does not own asset.".into())
        }
        let item_details = self.fetch_asset_details_internal(asset_id).await?;
        if item_details.is_public_domain.is_none() || !item_details.is_public_domain.unwrap() {
            return Err("Asset is not for sale.".into())
        } else if item_details.asset_type_id.is_none() || item_details.asset_type_id.unwrap() != structs::AssetType::Model {
            return Err("Asset type is not a Model.".into())
        } else if item_details.price_in_robux.is_some() && item_details.price_in_robux.unwrap() > 0 {
            return Err("Asset costs robux.".into())
        }

        self.purchase_asset_internal(asset_id).await?;
        Ok(())
    }

    pub async fn download_asset_bytes(&self, asset_id: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        self.download_asset_internal(asset_id).await
    }
}

mod internal {
    use reqwest::{header, Client};
    use surf::StatusCode;
    use crate::{utils, Backend}; 
    use super::structs::{AssetDeliveryResponse, AssetPurchaseReq, ItemDetails, RobloxApiError};

    const AUTH_URL: &str = "https://auth.roblox.com";
    const ASSETDELIVERY_URL: &str = "https://assetdelivery.roblox.com/v1";
    const ECONOMY_V1_URL: &str = "https://economy.roblox.com/v1";
    const ECONOMY_V2_URL: &str = "https://economy.roblox.com/v2";
    const INVENTORY_URL: &str = "https://inventory.roblox.com/v1";
    const XCSRF_HEADER: &str = "x-csrf-token";

    impl Backend {
        pub(super) fn prepare_headers(&self) -> header::HeaderMap {
            let mut reqwest_headers = header::HeaderMap::new();
    
            // send help
            let xcsrf_header = header::HeaderValue::from_static(utils::string_to_static_str(self.roblox_xcsrf_token.to_owned()));
            reqwest_headers.insert(XCSRF_HEADER, xcsrf_header);
            let mut cookie_header = header::HeaderValue::from_static(utils::string_to_static_str(format!(".ROBLOSECURITY={}", self.roblox_cookie.to_owned())));
            cookie_header.set_sensitive(true);
            reqwest_headers.insert(header::COOKIE, cookie_header);
    
            reqwest_headers
        }
    
        pub(crate) async fn refresh_xcsrf_token(&mut self) -> Result<(), Box<dyn std::error::Error>> {
            let request_result = Client::new()
                .post(AUTH_URL)
                .headers(self.prepare_headers())
                .send()
                .await?;
    
            let xcsrf = request_result
                .headers()
                .get(XCSRF_HEADER)
                .map(|x| x.to_str().unwrap().to_string())
                .unwrap();
    
            self.roblox_xcsrf_token = xcsrf;
            Ok(())
        }

        pub(super) async fn download_asset_internal(&self, asset_id: u64) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
            let formatted_url = format!(
                "{}/asset?id={}",
                ASSETDELIVERY_URL,
                asset_id
            );

            let mut cdn_redirect_response = surf::get(formatted_url)
                .header(XCSRF_HEADER, self.roblox_xcsrf_token.clone())
                .header("Cookie", format!(".ROBLOSECURITY={}", self.roblox_cookie.clone()))
                .send()
                .await?;

            if cdn_redirect_response.status() != 302 {
                let status_code = cdn_redirect_response.status();
                return match cdn_redirect_response.body_json::<RobloxApiError>().await {
                    Ok(info) => {
                        match info.errors.first() {
                            Some(err) => Err(format!("Roblox returned error code: {}, message: {}", status_code, err.message.clone()).into()),
                            None => Err(format!("Roblox returned error code: {}", status_code).into())
                        }
                    },
                    Err(_) => Err(format!("Roblox returned error code: {}", status_code).into())
                }
            }

            let location = match cdn_redirect_response.header("Location") {
                Some(location) => location.as_str(),
                None => return Err("Roblox did not return location for asset.".into())
            };

            let mut request_result = surf::get(location)
                .header(XCSRF_HEADER, self.roblox_xcsrf_token.clone())
                .header("Cookie", format!(".ROBLOSECURITY={}", self.roblox_cookie.clone()))
                .send()
                .await?;

            match request_result.status() {
                StatusCode::Ok => {},
                _ => {
                    let status_code = request_result.status();
                    return match request_result.body_json::<RobloxApiError>().await {
                        Ok(info) => {
                            match info.errors.first() {
                                Some(err) => Err(format!("Roblox returned error code: {}, message: {}", status_code, err.message.clone()).into()),
                                None => Err(format!("Roblox returned error code: {}", status_code).into())
                            }
                        },
                        Err(_) => Err(format!("Roblox returned error code: {}", status_code).into())
                    }
                }
            };

            let bytes: Vec<u8> = request_result.body_bytes().await?;
            Ok(bytes)
        }
    
        pub(super) async fn user_own_asset_internal(&self, user_id: u64, asset_id: u64) -> Result<bool, Box<dyn std::error::Error>> {
            let formatted_url = format!(
                "{}/users/{}/items/Asset/{}/is-owned",
                INVENTORY_URL,
                user_id,
                asset_id
            );
    
            let request_result = Client::new()
                .get(formatted_url)
                .headers(self.prepare_headers())
                .send()
                .await?;
    
            match request_result.text().await.unwrap_or(String::new()).parse::<bool>() {
                Ok(res) => Ok(res),
                Err(_) => Ok(false)
            }
        }
    
        pub(super) async fn fetch_asset_details_internal(&self, asset_id: u64) -> Result<ItemDetails, Box<dyn std::error::Error>> {
            let formatted_url = format!(
                "{}/assets/{}/details",
                ECONOMY_V2_URL,
                asset_id
            );
    
            let request_result = Client::new()
                .get(formatted_url)
                .headers(self.prepare_headers())
                .send()
                .await?;

            Ok(request_result.json::<ItemDetails>().await?)
        }
    
        pub(super) async fn purchase_asset_internal(&self, asset_id: u64) -> Result<(), Box<dyn std::error::Error>> {
            let formatted_url = format!(
                "{}/purchases/products/{}",
                ECONOMY_V1_URL,
                asset_id
            );
    
            let request_body = AssetPurchaseReq {
                expected_currency: 1,
                expected_price: 0,
            };
    
            Client::new()
                .post(formatted_url)
                .headers(self.prepare_headers())
                .json(&request_body)
                .send()
                .await?;
    
            Ok(())
        }
    }
}