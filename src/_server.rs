// pub use serde::{Deserialize, Serialize};
// pub use serde_json;
// pub use std::net::SocketAddr;
// use axum::{response::{IntoResponse, Json}, routing::get, Router};
// pub use crate::api_client::{ApiResult, BeaconApiClient, ApiResponseData, ApiClientError};
// pub use crate::timer::Timer;
// pub use crate::settings::Settings;
// pub use std::time::Instant;
// pub use crate::light_client_types::{Attestation, BlockHeaderData, CommitteeData, EthSpec, Hash256, LightClientUpdate};

// pub struct Node { 
//     base_url: String
// }

// pub struct BeaconApiServer {
//     timer: Timer,
//     config: Settings,
//     client: BeaconApiClient,
//     node: Node
// }

// impl BeaconApiServer {
//     pub fn from_config(config: &Settings) -> Self {
//         let timer: Timer = Timer::new(config.beacon_chain.genesis_time, config.beacon_chain.seconds_per_slot, config.beacon_chain.slots_per_epoch);
//         let node: Node = Node { base_url: config.node.url.to_string() };
//         let client: BeaconApiClient = BeaconApiClient::new(&config.node.url);
//         let settings = config.clone();

//         Self {
//             timer,
//             config: settings,
//             client,
//             node
//         }
//     }

//     pub async fn run(&self) {

//         let app = Router::new()
//             .route("/eth/v1/beacon/headers/head", get(serve_latest_header));

//         let port = self.config.server.port;
//         let addr = SocketAddr::from(([127, 0, 0, 1], port));
//         log::trace!("Server listening on {}", addr);

//         axum::Server::bind(&addr)
//             .serve(app.into_make_service())
//             .await
//             .unwrap();
//     }

//     pub async fn get_latest_header(&self) -> ApiResult<BlockHeaderData> {
//         self.client.get_latest_header().await
//     }
    
// }
// async fn handle() -> &'static str {
//     "test"
// }

// async fn serve_latest_header() -> Result<ApiResponseData<String>, reqwest::Error> {
//     // let data = BeaconApiServer.client.get_latest_header().await.unwrap();
//     // println!("{:?}", data);
//     // println!("{:?}", Json(data));
//     // let test = String::from("test");
//     // let request = Test {inner: "test"};
//     // let response = serde_json::ser::to_string(&request);
//     // println!("{:?}", response);
//     Ok(ApiResponseData {data: String::from("test") })
// }

// #[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
// pub struct Test {
//     inner: &'static str
// }