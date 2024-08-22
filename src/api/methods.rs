use super::structs::MBZSearchResponse;
use crate::BASE_BRAINZ_URL;

pub async fn get_song_data(
    api_client: reqwest::Client,
    song: String,
) -> Result<MBZSearchResponse, Box<dyn std::error::Error>> {
    let song = urlencoding::encode(&song);
    let url = format!("{}/release/?query={}", BASE_BRAINZ_URL, song);
    let response = api_client.get(url).send().await?;
    let json = response.json::<MBZSearchResponse>().await?;
    println!("{:?}", json);
    Ok(json)
}

pub fn find_song_name(track_name: String) -> String {
    let song_name = track_name;
    let song_name = song_name.replace(" ", "+");

    song_name
}

// async fn download_album_cover(api_client: reqwest::Client, api_key: String, album: String, artist: String) -> Result<(), Box<dyn std::error::Error>> {
//     let url = format!("/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json", api_key, artist, album);
//     let response = api_client.get(&url).send().await;
//     let json = response.unwrap().json();
//     println!("{:?}", json);
//     Ok(json.d)
// }
