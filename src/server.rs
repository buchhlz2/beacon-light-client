use serde::Serialize;
use serde_json;
use std::net::SocketAddr;
use warp::{Reply, Rejection, Filter};

// pub struct Server {
//     state: Arc<>
// }

// pub async fn get_latest_headers(state: Arc<State>) -> Result<impl warp::Reply, warp::Rejection> {
//     let status = state.chain.get_status();
//     Ok(warp::reply::json(&status))
// }

// pub struct LightClientServer {

// }