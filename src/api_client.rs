use reqwest::{Client, Error as HttpError};
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::{self, Error as JSONError};
use thiserror::Error;

use crate::light_client_types::{LightClientUpdate, BeaconBlockHeader};
use crate::timer::Timer;

const API_PREFIX: &str = "eth";
const ACCEPT_HEADER: &'static str = "Accept";
const ACCEPT_HEADER_VALUE: &'static str = "application/json";

#[derive(Error, Debug)]
pub enum ApiClientError {
    #[error("API error: {0}")]
    APIError(String),
    #[error("http error: {0}")]
    HttpClient(#[from] HttpError),
    #[error("json error: {0}")]
    SerdeError(#[from] JSONError),
}

type ApiResult<T> = Result<T, ApiClientError>;

#[derive(Clone, Debug)]
pub struct BeaconLightClient {
    http_client: Client,
    base_url: String
}

impl BeaconLightClient {
    pub async fn new(base_url: &str) -> Self {
        Self {
            http_client: Client::new(),
            base_url: base_url.to_string()
        }
    }
    
    pub async fn get_block_header(&self) -> ApiResult<BeaconBlockHeader> { 
        let endpoint = format!("{}", self.base_url);
        let response = self.http_client.get(&endpoint).header(ACCEPT_HEADER, ACCEPT_HEADER_VALUE).send().await?;

        println!("Status: {}", response.status());
        println!("{:?}", response);
        
        let body = response.text().await?;
        println!("Body:\n\n{}", body);

        let result: BeaconBlockHeader = serde_json::from_str(&body).unwrap();
        println!("Serde result:\n\n{}", result.id);
        Ok(result)
    }
    
    pub async fn get_light_client_update(t: Timer) -> Result<(), HttpError> {
        let (slot, epoch) = t.tick_slot().await;
        log::trace!("epoch: {}, slot: {}", epoch, slot);
        Ok(())
    }
}