mod db;
mod play;
mod record;
use clap::{App, Arg};
use std::path::Path;

/// CONSTANTS
const MODE: &str = "mode";
const SHORT_MODE: &str = "m";

const OUT_DIR: &str = "outdir";
const SHORT_OUT_DIR: &str = "o";

const RECORD_FILE: &str = "record_file";
const SHORT_RECORD_FILE: &str = "f";

const LOOP_COMMAND: &str = "loop";
const SHORT_LOOP_COMMAND: &str = "l";

fn main() {
    let default_outdir = std::env::current_dir().unwrap().join("recordings");

    std::fs::create_dir_all(&default_outdir).unwrap();

    let matches = App::new("Mouse and Keyboard Actions")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Darwin Subramaniam, darwinsubramaniam@gmail.com")
        .about("Records and replay mouse and keyboard actions")
        .arg(
            Arg::with_name(MODE)
                .short(SHORT_MODE)
                .default_value("record")
                .value_name("mode")
                .display_order(0)
                .possible_values(&["record", "r", "play", "p"])
                .help("Select Record or Play mode.")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name(OUT_DIR)
                .default_value(default_outdir.to_str().unwrap())
                .short(SHORT_OUT_DIR)
                .help("Path of directory to store the recording")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(RECORD_FILE)
                .short(SHORT_RECORD_FILE)
                .help("Path for the recording file.")
                .required_ifs(&[(SHORT_MODE, "p"), (MODE, "play")])
                .takes_value(true),
        )
        .arg(
            Arg::with_name(LOOP_COMMAND)
                .short(SHORT_LOOP_COMMAND)
                .help("Run this play in loop mode")
                .multiple(false),
        )
        .get_matches();

    initialize_engine(&matches);

    std::process::exit(0);
}

fn initialize_engine(matches: &clap::ArgMatches) {
    let mode = matches.value_of(MODE).expect("No Status Value");
    let outdir = matches.value_of(OUT_DIR).unwrap();
    let is_loop = matches.occurrences_of(LOOP_COMMAND) > 0;

    let outdir_path = Path::new(outdir);

    if mode == "play" || mode == "p" {
        let file_path = matches.value_of(RECORD_FILE).expect("Wrong file path");

        let file_path = Path::new(file_path);

        play::action(file_path, is_loop);
    }

    if mode == "record" || mode == "r" {
        println!("Start Recording");
        record::action(outdir_path);
    }
}
