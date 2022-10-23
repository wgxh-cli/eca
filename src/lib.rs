#![feature(result_option_inspect)]

pub mod models;
pub mod apis;

pub async fn setup_server() {
  warp::serve(apis::index()).run(([127, 0, 0, 1], 11451)).await;
}
