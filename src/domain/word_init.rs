// use log::info;
use std::path::Path;

use crate::utils::letters_from_yaml::load;
use crate::utils::insert_in_sled;
use crate::models::{Database, AppConfig};

pub fn init(dbs: &Database, config: &AppConfig) {
    let (letters, yaml_path) = load(config).expect("Failed to load YAML file");

    let yaml_directory = Path::new(&yaml_path).parent().unwrap_or_else(|| Path::new(""));

    for mut letter in letters {
        let key = letter.id.clone();

        // If there's an audio file, prepend the relative directory path
        if let Some(audio) = letter.audio.clone() {
            // info!("Inserting audio path: {:?}", audio);
            let audio_path = yaml_directory.join(&audio).to_string_lossy().to_string();
            // info!("Inserting letter.audio: {:?}", audio_path);
            letter.audio = Some(audio_path);
        }
        // info!("Inserting letter with key: {:?}", key);
        insert_in_sled::insert_in_sled(&dbs.word_db, &key, &letter);
    }
}