mod db;
mod play;
mod record;
use clap::{App, Arg};
use std::path::Path;

/// CONSTANTS
const STATUS: &str = "status";
const SHORT_STATUS: &str = "s";

const OUT_DIR: &str = "outdir";
const SHORT_OUT_DIR: &str = "o";

const RECORD_FILE: &str = "record_file";
const SHORT_RECORD_FILE: &str = "f";

const LOOP_COMMAND: &str = "loop";
const SHORT_LOOP_COMMAND: &str = "l";

fn main() {
    let default_outdir = std::env::current_dir().unwrap().join("recordings");
    std::fs::create_dir_all(&default_outdir).unwrap();
    let default_outdir_str = default_outdir.to_str().unwrap();

    let matches = build_app(default_outdir_str).get_matches();
    initialize_engine(&matches);
    std::process::exit(0);
}

fn build_app(default_outdir: &str) -> App {
    App::new("Mouse and Keyboard Actions")
        .version("0.1.0")
        .author("darwin subramaniam")
        .about("Records and replay mouse and keyboard actions")
        .arg(build_status_arg())
        .arg(build_outdir_arg(default_outdir))
        .arg(build_record_file_arg())
        .arg(build_loop_command_arg())
}

fn build_status_arg() -> Arg {
    Arg::with_name(STATUS)
        .short(SHORT_STATUS)
        .default_value("record")
        .value_name("status")
        .display_order(0)
        .possible_values(&["record", "r", "play", "p"])
        .help("Define Record or Play")
        .takes_value(true)
        .required(true)
}

fn build_outdir_arg(default_outdir: &str) -> Arg {
    Arg::with_name(OUT_DIR)
        .default_value(default_outdir)
        .short(SHORT_OUT_DIR)
        .help("Path of directory to store the recording")
        .takes_value(true)
}

fn build_record_file_arg() -> Arg {
    Arg::with_name(RECORD_FILE)
        .short(SHORT_RECORD_FILE)
        .help("Define the record file")
        .required_ifs(&[(SHORT_STATUS, "p"), (STATUS, "play")])
        .takes_value(true)
}

fn build_loop_command_arg() -> Arg {
    Arg::with_name(LOOP_COMMAND)
        .short(SHORT_LOOP_COMMAND)
        .help("Flag : Run this play in loop mode")
        .multiple(false)
}


fn initialize_engine(matches: &clap::ArgMatches) {
    let status = matches.value_of(STATUS).expect("No Status Value");
    let outdir = matches.value_of(OUT_DIR).unwrap();
    let is_loop = matches.occurrences_of(LOOP_COMMAND) > 0;

    let outdir_path = Path::new(outdir);

    match status {
        "play" | "p" => {
            let file_path = matches.value_of(RECORD_FILE).expect("Wrong file path");
            let file_path = Path::new(file_path);
            play::action(file_path, is_loop);
        },
        "record" | "r" => {
            println!("Start Recording");
            record::action(outdir_path);
        },
        _ => panic!("Invalid status value"),
    }
}
