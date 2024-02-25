use mongodb::Database;

use crate::Backend;

pub mod api_keys;
pub mod moderation;

impl Backend {
    pub fn get_database(&self) -> Database {
        if Option::is_none(&self.mongo_client) {
            panic!("Database not connected!")
        }
        self.mongo_client.as_ref().unwrap().default_database().unwrap()
    }
}