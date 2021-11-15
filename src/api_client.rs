use reqwest::{Client, Error as HttpError};
use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
use serde_json::{self, Error as JsonError, Value, Map};
use thiserror::Error;
use std::fmt;
pub use crate::light_client_types::{BlockHeaderData};
pub use crate::timer::Timer;
pub use hex;

const API_PREFIX: &str = "eth";
const ACCEPT_HEADER: &'static str = "Accept";
const ACCEPT_HEADER_VALUE: &'static str = "application/json";

#[derive(Error, Debug)]
pub enum ApiClientError {
    #[error("API error: {0}")]
    ApiError(String),
    #[error("http error: {0}")]
    HttpClient(#[from] HttpError),
    #[error("json error: {0}")]
    SerdeError(#[from] JsonError),
}

type ApiResult<T> = Result<T, ApiClientError>;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct ApiResponseData<T> {
    pub data: T
}

impl<T: Serialize + DeserializeOwned> From<T> for ApiResponseData<T> {
    fn from(data: T) -> Self {
        Self { data }
    }
}

#[derive(Clone, Debug)]
pub struct BeaconLightClient {
    http_client: Client,
    base_url: String
}

pub async fn get_call<T: Serialize + DeserializeOwned>(client: &Client, endpoint: &str) -> ApiResult<T> { 
    let response = client.get(endpoint).header(ACCEPT_HEADER, ACCEPT_HEADER_VALUE).send().await?;
    let body = response.bytes().await?;
    let result = serde_json::from_slice::<ApiResponseData<T>>(&body).map(|resp| resp.data);
    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err.into())
    }
}

impl BeaconLightClient {
    pub async fn new(base_url: &String) -> Self {
        Self {
            http_client: Client::new(),
            base_url: base_url.to_string() + "/" + API_PREFIX
        }
    }
    
    pub async fn get_block_header(&self) -> ApiResult<BlockHeaderData> { 
        let endpoint = format!("{}/v1/beacon/headers", self.base_url);
        let result = get_call::<Vec<BlockHeaderData>>(&self.http_client, &endpoint).await?;
        let block_header_data = result.into_iter().nth(0);
        match block_header_data {
            Some(block_header_data) => Ok(block_header_data),
            None => Err(ApiClientError::ApiError("Error retrieving block header".to_string()))
        }
    }

    // pub async fn get_light_client_update(&self) -> ApiResult<LightClientUpdate> {
    //     // let endpoint = format!("{}/v1/lightclient/best_update/:periods", self.base_url);
    //     let endpoint = format!("{}", self.base_url);
    //     let result: LightClientUpdate = LightClientUpdate {
    //         header: self.get_block_header().await?,
    //         next_sync_committee: String::from("committee"),
    //         next_sync_committee_branch: vec!(Hash256::random()),
    //         finality_header: None,
    //         finality_branch: None,
    //         sync_committee_bits: vec![0,1,0,1],
    //         sync_committee_signature: String::from("signature"),
    //         fork_version: [1,2,3,4]
    //     };
    //     Ok(result)
    // }
}

fn decode_string_hex_to_bytes(hex: String) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    hex::decode_to_slice(hex.trim_start_matches("0x"), &mut bytes).unwrap();
    return bytes
}