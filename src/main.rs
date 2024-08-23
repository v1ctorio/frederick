use audiotags::{Album, Tag};
use clap::Parser;
use constants::generate_configuration;
use owo_colors::OwoColorize;
use reqwest::header::{self, ACCEPT};
use serde::Deserialize;
use std::fs;
use toml::value::Array;
use urlencoding::encode;

mod constants;

mod api;
use api::client::build_api_client;
use api::methods::*;

const supported_file_extensions: [&str; 3] = ["mp3", "flac", "m4a"];
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Frederick {
    #[arg(short, long)]
    file: Option<String>,
    //file: std::path::PathBuf,
    #[arg(short, long, default_value = "false")]
    generate_config_file: bool,
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

#[tokio::main]
async fn main() {
    let api_client = build_api_client();

    let args = Frederick::parse();

    if args.generate_config_file {
        generate_configuration().await.unwrap();
        return;
    }

    let file = args
        .file
        .expect("No file provided. Use the --file flag to provide a flag.");
    let file = std::path::Path::new(&file);

    let file_name = file.file_stem().expect("File to obtain file name").to_str().unwrap().to_string();
    let file_extension = file.extension().expect("Failed to retrive file extension").to_str().unwrap();

    println!("The file name is {:?}", file_name);
    println!("The file extension is {:?}", file_extension);

    if !file.is_file()
        || !supported_file_extensions.contains(&file.extension().unwrap().to_str().unwrap())
    {
        println!("{}", "The path provided is not valid.".red());
        return;
    }


    let found_data = get_song_data(
        api_client,
        file.file_stem().unwrap().to_str().unwrap().to_string(),
    );

    let found_data = found_data.await.unwrap();

    println!(
        "The file extracted song name (from  filename) is {:?}",
        &file
    );

    println!("The following data has been found {:?}", found_data);

    let chosen_release = &found_data.releases.first().unwrap();

    let mut new_tag = Tag::new().read_from_path(file).unwrap();
    new_tag.set_title(&chosen_release.title);
    new_tag.set_year(chosen_release.date.parse().unwrap());

    //println!("The file has been tagged with the new data.");
}
