use audiotags::{Album, Tag};
use clap::Parser;
use dirs_next::config_dir;
use reqwest::header::{self, ACCEPT};
use std::fs;
use urlencoding::encode;
use serde::Deserialize;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Frederick {
    #[arg(short, long)]
    file: String,
    //file: std::path::PathBuf,
}


#[derive(Debug,Deserialize)]
struct MBZSearchResponse {
    created: String,
    count: u32,
    offset: u32,
    releases: Vec <Release>,
}

#[derive(Debug,Deserialize)]
struct Release {
    id: String,
}


struct TrackData {
    title: String,
    artist: String,
    album: String,
    year: i32,
    cover: Option<String>,
}

const BASE_BRAINZ_URL: &str = "https://musicbrainz.org/ws/2";

//TODO: retrive data from the last.fm API
const TITLE: &str = "Hello chat";
const ARTIST: &str = "Frederick";
const ALBUM: &str = "Frederick's album";
const YEAR: i32 = 1969;

fn main() {
    let api_client = build_api_client();

    #[allow(non_snake_case)]
    let LASTFM_API_KEY: String = std::env::var("LASTFM_API_KEY").unwrap();

    let args = Frederick::parse();
    println!("Hello, chat. The file provided is: {:?}", &args.file);
    let meta = fs::metadata(&args.file);
    println!("The file metada is {:?}", meta.unwrap());

    let mut audio_tag = Tag::new().read_from_path(&args.file).unwrap();

    let current_track_title = audio_tag.title().unwrap();

    let new_track_data: TrackData = TrackData {
        title: find_song_name(current_track_title.to_string()),
        artist: ARTIST.to_string(),
        album: ALBUM.to_string(),
        year: YEAR,
        cover: None,
    };

    audio_tag.set_title(TITLE);
    audio_tag.set_artist(ARTIST);
    audio_tag.set_album(Album {
        title: ALBUM,
        artist: Some(ARTIST),
        cover: None,
    });
    audio_tag.set_year(YEAR);

    audio_tag.write_to_path(&args.file).unwrap();
    println!("The file has been tagged with the new data.");
}

fn build_api_client() -> reqwest::ClientBuilder {
    let client_contact: String = get_user_contact();

    let mut headers = header::HeaderMap::new();

    headers.insert(ACCEPT, "application/json".parse().unwrap());
    headers.insert(
        header::USER_AGENT,
        format!("{}{}", "Frederick /".to_string(), client_contact)
            .parse()
            .expect("Error setting the user agent"),
    );

    let api_client: reqwest::ClientBuilder = reqwest::Client::builder().default_headers(headers);

    api_client
}

fn get_user_contact() -> String {
    let contact = std::env::var("MBRAINZ_CONTACT").unwrap_or_else(|_| {
        if let Some(config_dir) = config_dir() {
            println!("Config directory: {}", config_dir.display());
            // You can now use `config_dir` to store your dotfiles
        } else {
            println!("Could not determine the config directory.");
        }
        "contact".to_string()
    });
    contact
}

async fn get_song_data(
    api_client: reqwest::Client,
    song: String,
) -> Result<(MBZSearchResponse), Box<dyn std::error::Error>> {
    let song = urlencoding::encode(&song);
    let url = format!("{}/release/?query={}", BASE_BRAINZ_URL, song);
    let response = api_client.get(url).send().await?;
    let json = response.json::<MBZSearchResponse>().await?;
    println!("{:?}", json);
    Ok(json)
}

fn find_song_name(track_name: String) -> String {
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
