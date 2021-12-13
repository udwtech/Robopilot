use chrono::Local;
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Error, Read},
    ops::Add,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct RecordDb {
    pub created_at: i64,
    pub version:Option<i32>,
    pub recordings: Vec<Recording>,
    #[serde(skip, default)]
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
    /// Define the Version of the Database 
    const VERSION_DB:i32=1;

    pub fn load_db(file_path: &Path) -> RecordDb {
        let mut option = fs::OpenOptions::new();

        let mut file = option
            .read(true)
            .open(file_path)
            .expect("Unable to open the file");

        let mut content: String = String::new();
        let _size = file
            .read_to_string(&mut content)
            .expect("Unable to load file");

        let db: RecordDb = toml::from_str(&content).expect(
            "
            Content of the file is not as expected",
        );

        db
    }

    pub fn add(&mut self, record: Recording) {
        self.recordings.push(record);
    }

    // Save the recording to the
    pub fn save_all(&self) {
        let file_copy = self.file.clone();

        let content = toml::to_string(&self).expect("Fail to serialize data");

        fs::write(file_copy, content).expect("Fail to save recording");
    }

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
                    created_at: Local::now().timestamp(),
                    version: Some(RecordDb::VERSION_DB),
                    recordings: Vec::new(),
                    file: file_path,
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
