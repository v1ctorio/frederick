use reqwest::header::{self, ACCEPT, USER_AGENT};

use crate::constants::get_user_contact;
pub fn build_api_client() -> reqwest::Client {
    let client_contact: String = get_user_contact();

    let mut headers = header::HeaderMap::new();

    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(
        USER_AGENT,
        format!("{}{}", "Frederick /".to_string(), client_contact)
            .parse()
            .expect("Error setting the user agent"),
    );

    let api_client: reqwest::Client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap();

    api_client
}
