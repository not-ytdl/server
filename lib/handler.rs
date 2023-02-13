use actix_web::web;
use ytdlrs_lib::api::client;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Search {
  id: String,
}
pub struct Routes;

impl Routes {
  pub async fn search(search: web::Query<Search>) -> String {
    format!("The id is: {}", search.id)
  }
}