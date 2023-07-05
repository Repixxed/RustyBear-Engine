use std::io::BufReader;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct ThemeConfiguration {
    pub background_music: String,
}

impl Default for ThemeConfiguration {

    fn default() -> Self {
        ThemeConfiguration { background_music: "default.mp3".to_string() }
    }
}

pub fn load_themes() -> ThemeConfiguration
{
    let themes_folder = std::path::Path::new("themes");
    let themes_config = themes_folder.join("config.json");

    if let Err(e) = std::fs::create_dir_all(themes_folder) 
    {
        log::error!("Could not create themes directory. Message: {}. Defaulting... ", e);
        return ThemeConfiguration::default();
    }

    let file = std::fs::File::open(themes_config.clone());

    if file.is_err()
    {
        log::warn!("Could not access {}. Creating and defaulting...", themes_config.to_str().unwrap_or("INVALID_PATH"));

        let default = ThemeConfiguration::default();

        if let Err(e) = std::fs::write(themes_config.clone(), serde_json::to_string(&default).unwrap_or("{}".to_string()))
        {
            log::error!("Could not create {}. {}", themes_config.to_str().unwrap_or("INVALID_PATH"), e);
        }
        
        return default;
    }

    let reader = BufReader::new(file.unwrap());

    let conf = serde_json::from_reader(reader);

    if conf.is_err()
    {
        log::error!("Failed to parse {}. Defaulting...", themes_config.to_str().unwrap_or("INVALID_PATH"));
        return ThemeConfiguration::default();
    }
   
    conf.unwrap()
}