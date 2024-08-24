use audiotags::{Album, Tag};
use clap::Parser;
use constants::generate_configuration;
use owo_colors::OwoColorize;
use reqwest::header::{self, ACCEPT};
use serde::Deserialize;
use std::fs::File;
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
    #[arg(short, long)]
    release_name: Option<String>,
}

struct TrackData {
    title: String,
    artist: String,
    album: String,
    year: i32,
    cover: Option<String>,
}

pub const BASE_BRAINZ_URL: &str = "https://musicbrainz.org/ws/2";



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


    let mut song_name = file.file_stem().expect("File to obtain file name").to_str().unwrap().to_string();
    let file_extension = file.extension().expect("Failed to retrive file extension").to_str().unwrap();

    if let Some(n) = args.release_name {
        song_name = n;
    }

    println!("The release name im searching for is {:?}", song_name.on_blue());
    println!("The file extension is {:?}", file_extension);

    if !file.is_file()
        || !supported_file_extensions.contains(&file.extension().unwrap().to_str().unwrap())
    {
        println!("{}", "The path provided is not valid.".red());
        return;
    }


    let found_data = get_song_data(
        api_client,
        song_name,
    );

    let found_data = found_data.await.unwrap();

    println!(
        "The file I'm going to edit is {:?}",
        &file.on_yellow()
    );

    let mut file_handle = File::open(&file).unwrap();


    if (&found_data.releases).is_empty() {
        println!("{}","No data found for the release name provided.".red());
        return;
    }


    let chosen_release = &found_data.releases.first().unwrap();
    println!("The following data has been found: title: {} year: {}, country: {}", &chosen_release.title.on_green(), &chosen_release.date.as_ref().unwrap().on_green(),&chosen_release.country.as_ref().unwrap().on_green());

    let mut new_tag = Tag::new().read_from_path(file).unwrap();
    new_tag.set_title(&chosen_release.title);
    new_tag.set_year(chosen_release.date.as_ref().unwrap()[0..4].parse().unwrap());

    new_tag.write_to(&mut file_handle).unwrap();
    println!("The file has been tagged with the new data.");
}
