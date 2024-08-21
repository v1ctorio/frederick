use clap::Parser;
use std::fs;
use audiotags::{Album, Tag};

#[derive(Parser, Debug)]

#[command(version, about, long_about = None)]
struct Frederick {
    #[arg(short, long)]
    file: String,
    //file: std::path::PathBuf,
}
//TODO: retrive data from the last.fm API
const TITLE: &str = "Hello chat";
const ARTIST: &str = "Frederick";
const ALBUM: &str = "Frederick's album";
const YEAR: i32 = 1969;

fn main() {

    let api_client = reqwest::Client::new();
    const LASTFM_API_KEY: String = std::env::var("LASTFM_API_KEY").unwrap();

    let args = Frederick::parse();
    println!("Hello, chat. The file provided is: {:?}", &args.file);
    let meta = fs::metadata(&args.file);
    println!("The file metada is {:?}", meta.unwrap());

    let mut audio_tag  = Tag::new().read_from_path(&args.file).unwrap(); 
    audio_tag.set_title(TITLE);
    audio_tag.set_artist(ARTIST);
    audio_tag.set_album(Album{
        title: ALBUM,
        artist: Some(ARTIST),
        cover: None,
    });
    audio_tag.set_year(YEAR);

    audio_tag.write_to_path(&args.file).unwrap();    
    println!("The file has been tagged with the new data.");

}


fn get_song_data(api_client: reqwest::Client, api_key: String, song: String, artist: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(" /2.0/?method=track.getInfo&api_key={}&artist={}&track={}&format=json", api_key, artist, song);
    let response = api_client.get(&url).send()?;
    let json = response.json()?;
    println!("{:?}", json);
    Ok(json)
}

fn find_song_name(track_name:String) -> String {
    let mut song_name = track_name;
    let mut song_name = song_name.replace(" ", "+");
    song_name
}

fn download_album_cover(api_client: reqwest::Client, api_key: String, album: String, artist: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!("/2.0/?method=album.getinfo&api_key={}&artist={}&album={}&format=json", api_key, artist, album);
    let response = api_client.get(&url).send()?;
    let json = response.json()?;
    println!("{:?}", json);
    Ok(json.d)
}