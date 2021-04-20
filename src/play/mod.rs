mod keyboard;
mod mouse;

use std::path::Path;
use crate::db::RecordDb;

pub fn action(record_file: &Path) {
    let db = RecordDb::load_db(record_file);

    db.recordings.iter().for_each(|recording| {
        let device = &recording.device;

        match device as &str {
            "mouse" => mouse::play(&recording),
            "keyboard" => keyboard::play(&recording),
            _ => println!("Unkown device recording was found {}", device),
        }
    })
}
