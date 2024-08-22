use dirs_next::config_dir;
use owo_colors::OwoColorize;

pub fn get_user_contact() -> String {
    let contact = std::env::var("MBRAINZ_CONTACT").unwrap_or_else(|_| {
        if let Some(config_dir) = config_dir() {
            println!(
                "MBRAINZ_CONTACT NOW FOUND, trying to read from {}/frederickrs/config.toml",
                config_dir.display()
            );

            let config_path = config_dir.join("frederickrs/config.toml");
            let config_str =
                std::fs::read_to_string(&config_path).unwrap_or_else(|_| {
                    panic!("{} {} {}","To use frederick, you must set the MBRAINZ_CONTACT environment variable or create a config file at".red() ,config_path.display().blue(), "with a 'contact' key. \n\n This is required by the MusicBrainz API".red());
                });
            let config: toml::Value =
                toml::from_str(&config_str).expect("Failed to parse config file");
            let contact = config
                .get("contact")
                .expect("Contact not found in config file")
                .as_str()
                .expect("Failed to convert contact to string")
                .to_string();
            contact
        } else {
            panic!("Failed to get the config directory");
        }
    });
    contact
}
