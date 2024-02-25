use mongodb::{Client, options::ClientOptions};
use id_converter::IDConverter;

pub mod roblox;
pub mod database;
pub mod luau;
mod id_converter;
mod utils;
pub struct Backend {
    pub(crate) roblox_cookie: String,
    pub(crate) roblox_xcsrf_token: String,
    pub(crate) id_generator: IDConverter,
    pub(crate) mongo_client: Option<Client>
}

impl Backend {
    #[allow(unused_must_use)]
    pub fn new(roblox_cookie: String, id_generator_alphabets: Vec<String>) -> Self {
        if id_generator_alphabets.len() < 2 {
            panic!("ID Generator must have at least 2 alphabets.");
        }
        let id_generator = IDConverter::new(&id_generator_alphabets[0], &id_generator_alphabets[1]);

        let mut backend_self = Self { roblox_cookie: roblox_cookie, roblox_xcsrf_token: String::new(), id_generator: id_generator, mongo_client: None };
        backend_self.refresh_xcsrf_token();

        backend_self
    } 
    
    pub async fn connect_mongodb(&mut self, mongodb_url: String, default_database: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let mut mongo_options = ClientOptions::parse(mongodb_url).await?;
        mongo_options.default_database = Some(default_database.unwrap_or("test".to_string()));
        let mongo_client = Client::with_options(mongo_options)?;

        self.mongo_client = Some(mongo_client);
        Ok(())
    }

    pub fn get_shareable_id(&self, id: String) -> Result<String, Box<dyn std::error::Error>> {
        let parsed_id = id.parse::<u64>();
        match parsed_id {
            Ok(i) => self.id_generator.to_short(i),
            Err(_) => Err("ID cannot be converted into integer.".into())
        }
    }

    pub fn get_number_id(&self, id: String) -> Result<u64, Box<dyn std::error::Error>> {
        self.id_generator.to_number(id)
    }
}