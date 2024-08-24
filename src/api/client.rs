use reqwest::header::{self, ACCEPT};

use crate::constants::get_user_contact;
pub fn build_api_client() -> reqwest::Client {
    let client_contact: String = get_user_contact();

    let mut headers = header::HeaderMap::new();

    headers.insert(ACCEPT, "application/json".parse().unwrap());
    

    let api_client: reqwest::Client = reqwest::Client::builder()
        .default_headers(headers)
        .user_agent(format!("Frederick /{}", client_contact))
        .build()
        .unwrap();

    api_client
}
