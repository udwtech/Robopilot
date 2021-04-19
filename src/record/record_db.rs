use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{fs::{self, File}, io::Error, ops::Add, path::{Path, PathBuf}};

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordDb {
    pub created_at: i64,
    pub recordings: Vec<Recording>,
    #[serde(skip,default)]
    file: PathBuf,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Recording {
    pub device: String,
    pub x: Option<i32>,
    pub y: Option<i32>,
    pub button_pressed: Option<Vec<bool>>,
    pub key_pressed: Option<Vec<String>>,
}

impl RecordDb {
    pub fn read_db(file_path: &Path) {}

    pub fn load_db(file_path: &Path){}

    pub fn add(&self, record: Recording) {}

    // Save the recording to the 
    pub fn save_all(&self){

    }

    pub fn delete(&self, record: Recording) {}

    pub fn new(outdir: &Path, filename: Option<&str>) -> Result<RecordDb, Error> {
        let current_file_count = count_total_recordings(outdir)?;

        let filename = match filename {
            Some(file) => [file, "toml"].join("."),
            None => {
                let newfile =
                    ["recording", current_file_count.add(1).to_string().as_ref()].join("_");

                [&newfile, "toml"].join(".")
            }
        };

        let file_path = Path::new(outdir).join(filename);

        match File::create(&file_path) {
            Err(err) => Err(err),
            Ok(created_file) => {
                println!("Successfully Created at {:?}", created_file.metadata());

                Ok(RecordDb {
                    file: file_path,
                    created_at: Local::now().timestamp(),
                    recordings: Vec::new(),
                })
            }
        }
    }
}

fn count_total_recordings(outdir: &Path) -> Result<usize, Error> {
    let a = fs::read_dir(outdir)?.count();

    println!("Total File Found {}", a);

    Ok(a)
}
