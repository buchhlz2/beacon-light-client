use reqwest::{Client, Error as HttpError};
use serde::de::DeserializeOwned;
pub use serde::{Deserialize, Serialize};
use serde_json::{self, Error as JsonError, Value};
use thiserror::Error;
use std::fmt;
pub use types::{Slot, Epoch, Hash256};
use crate::light_client_types::*;
use crate::timer::Timer;
pub use hex;

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

impl BeaconLightClient {
    pub async fn new(base_url: &String) -> Self {
        Self {
            http_client: Client::new(),
            base_url: base_url.to_string() + "/" + API_PREFIX
        }
    }
    
    pub async fn get_block_header(&self) -> Result<(), ApiClientError> { 
        let endpoint = format!("{}/v1/beacon/headers", self.base_url);
        let response = self.http_client.get(&endpoint).header(ACCEPT_HEADER, ACCEPT_HEADER_VALUE).send().await?;
        let body = response.bytes().await?;

        println!("Body:\n\n{:?}", body);    

        let result: Value = serde_json::from_slice(&body)?;
        let obj = result.as_object().unwrap();
        let data = &obj.get("data").unwrap().as_array().unwrap()[0].as_object().unwrap();
        let block_header_data: BlockHeaderData = BlockHeaderData {
            root: Hash256::from(decode_string_hex_to_bytes(data.get("root").unwrap().as_str().unwrap())),
            canonical: data.get("canonical").unwrap().as_bool().unwrap(),
            // header: data.get("header").unwrap()
        };

        println!("obj here {:?}\n", obj);
        println!("data here {:?}\n", data);
        println!("block_header_data here {:?}\n", block_header_data);
        // let result = serde_json::from_slice::<ApiResponseData<T>>(&body).map(|resp| resp.data);
        // let block_header_data: BlockHeaderData = result.unwrap().map(
        //     |data: BlockHeaderData| (data.root, data.canonical, data.header));
        // println!("Serde result:\n\n{:?}", block_header_data);
        Ok(())
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
    
    pub async fn light_client_update(t: Timer) -> Result<(), HttpError> {
        let (slot, epoch) = t.tick_slot().await;
        log::trace!("epoch: {}, slot: {}", epoch, slot);
        Ok(())
    }
}

fn decode_string_hex_to_bytes(hex: &str) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    hex::decode_to_slice(hex.trim_start_matches("0x"), &mut bytes).unwrap();
    return bytes
}