use reqwest::header::{self, ACCEPT, USER_AGENT};

use crate::constants::get_user_contact;
pub fn build_api_client() -> reqwest::ClientBuilder {
    let client_contact: String = get_user_contact();

    let mut headers = header::HeaderMap::new();

    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(
        USER_AGENT,
        format!("{}{}", "Frederick /".to_string(), client_contact)
            .parse()
            .expect("Error setting the user agent"),
    );

    let api_client: reqwest::ClientBuilder = reqwest::Client::builder().default_headers(headers);

    api_client
}
