mod keyboard;
mod mouse;

use std::path::Path;
use crate::db::RecordDb;

pub fn action(record_file: &Path, is_loop:bool) {
    let db = RecordDb::load_db(record_file);

    // Start Looping the below recording.
    if is_loop {
        println!("Start looping");
        loop {
            play(&db);
        }
    }else{
        println!("Start single play");
        play(&db);
    }

   
}

// Play the recording.
fn play(db: &RecordDb) {
    db.recordings.iter().for_each(|recording| {
        let device = &recording.device;
        match device as &str {
            "mouse" => mouse::play(&recording),
            "keyboard" => keyboard::play(&recording),
            _ => println!("Unkown device recording was found {}", device),
        }
    })
}
