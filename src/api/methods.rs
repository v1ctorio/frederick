use std::borrow::Borrow;

use super::structs::MBZSearchResponse;
use crate::{api::structs::MBZReleaseGroupResponse, BASE_BRAINZ_URL};

pub async fn get_song_data(
    api_client: reqwest::Client,
    song: String,
) -> Result<MBZSearchResponse, Box<dyn std::error::Error>> {
    let song = urlencoding::encode(&song);
    let url = format!("{}/release/?query={}", BASE_BRAINZ_URL, song);
    let response = api_client.get(url).send().await?;

    let cloned_response = response.borrow();

    println!("{:?}", &response.text().await);

    // let json = response.json::<MBZSearchResponse>().await?;
    // println!("{:?}", json);

    let debugMBZSR = MBZSearchResponse {
        created: "2021-08-29T17:00:00Z".to_string(),
        count: 1,
        offset: 0,
        releases: vec![],
    };
    //Ok(json)
    Ok(debugMBZSR)
}

pub fn find_song_name(track_name: String) -> String {
    let song_name = track_name;
    let song_name = song_name.replace(" ", "+");

    song_name
}

pub async fn find_release_group(id: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("{}/release-group/{}", BASE_BRAINZ_URL, id);
    let response = reqwest::get(&url).await?;
    let json = response.json::<MBZReleaseGroupResponse>().await?;
    println!("{:?}", json);
    Ok(())
}

// async fn download_album_cover(api_client: reqwest::Client, api_key: String, album: String, artist: String) -> Result<(), Box<dyn std::error::Error>> {
//     let url = format!("/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json", api_key, artist, album);
//     let response = api_client.get(&url).send().await;
//     let json = response.unwrap().json();
//     println!("{:?}", json);
//     Ok(json.d)
// }
