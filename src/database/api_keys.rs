use mongodb::{bson::doc, Collection};
use serde::{ Deserialize, Serialize };

use crate::Backend;
use crate::IDConverter;
use crate::utils::datetime_now;

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiKey {
    pub value: String,
    #[serde(rename = "assignOwner")]
    pub assign_owner: String,
    #[serde(rename = "associatedDiscordUser")]
    pub associated_discord_user: Option<String>,
    pub enabled: bool,
    #[serde(rename = "timeCreated")]
    pub time_created: f64,
}

impl Backend {
    pub(super) async fn generate_api_key(&self) -> Result<String, Box<dyn std::error::Error>> {
        let database = self.get_database();
        let api_keys_collection: Collection<ApiKey> = database.collection("apikeys");
        
        // my reaction when rust
        let api_key_generator: IDConverter = IDConverter::new(
            &"qwertyuiopasdfghjklzxcvbnm0192837465".to_string(),
            &"5432189076".to_string()
        );

        let doc_count: u64 = api_keys_collection.count_documents(None, None).await?;
        api_key_generator.to_short(doc_count * 8 + datetime_now() * 2)
    }

    pub async fn find_api_key_entry(&self, api_key: &str) -> Result<Option<ApiKey>, Box<dyn std::error::Error>> {
        let database = self.get_database();

        let api_keys_collection: Collection<ApiKey> = database.collection("apikeys");

        let result = api_keys_collection.find_one(
            doc! { 
                "value": api_key.to_string()
            },
            None
        ).await?;

        Ok(result)
    }

    pub async fn api_key_entry_exist(&self, api_key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let result = self.find_api_key_entry(api_key).await?;

        match result {
            Some(_) => Ok(true),
            None => Ok(false)
        }
    }

    pub async fn create_api_key_entry(&self) -> Result<(), Box<dyn std::error::Error>> {
        let database = self.get_database();
        
        let api_keys_collection: Collection<ApiKey> = database.collection("apikeys");
        let new_api_key = self.generate_api_key().await?;

        let doc = ApiKey {
            value: new_api_key,
            assign_owner: "None".to_string(),
            associated_discord_user: None,
            enabled: true,
            time_created: datetime_now() as f64
        };
        api_keys_collection.insert_one(doc, None).await?;
        Ok(())
    }

    pub async fn search_api_key_entries_with_roblox_id(&self, roblox_id: u64) -> Result<Option<ApiKey>, Box<dyn std::error::Error>> {
        let database = self.get_database();
        
        let api_keys_collection: Collection<ApiKey> = database.collection("apikeys");

        let result = api_keys_collection.find_one(
            doc! { 
                "assignOwner": roblox_id.to_string()
            },
            None
        ).await?;

        Ok(result)
    }

    pub async fn search_api_key_entries_with_discord_id(&self, discord_id: u64) -> Result<Option<ApiKey>, Box<dyn std::error::Error>> {
        let database = self.get_database();
        
        let api_keys_collection: Collection<ApiKey> = database.collection("apikeys");

        let result = api_keys_collection.find_one(
            doc! { 
                "associatedDiscordUser": Some(discord_id.to_string())
            },
            None
        ).await?;

        Ok(result)
    }

    pub async fn is_valid_api_key(&self, api_key: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let api_key_entry = self.find_api_key_entry(api_key).await?;
        match api_key_entry {
            Some(entry) => Ok(entry.enabled == true),
            None => Ok(false)
        }
    }
}