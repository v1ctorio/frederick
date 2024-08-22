use audiotags::{Album, Tag};
use clap::Parser;
use reqwest::header::{self, ACCEPT};
use serde::Deserialize;
use std::fs;
use urlencoding::encode;

mod constants;

mod api;
use api::client::build_api_client;
use api::methods::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Frederick {
    #[arg(short, long)]
    file: String,
    //file: std::path::PathBuf,
}

struct TrackData {
    title: String,
    artist: String,
    album: String,
    year: i32,
    cover: Option<String>,
}

pub const BASE_BRAINZ_URL: &str = "https://musicbrainz.org/ws/2";

//TODO: retrive data from the last.fm API
const TITLE: &str = "Hello chat";
const ARTIST: &str = "Frederick";
const ALBUM: &str = "Frederick's album";
const YEAR: i32 = 1969;

fn main() {
    let api_client = build_api_client();

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
