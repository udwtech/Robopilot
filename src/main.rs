mod play;
mod record;
use std::{io::Error, path::Path};

use chrono::{Date, DateTime, Local};

use clap::{App, Arg};

fn main() {
    let now = Local::now();

    print!("{:?}", now);

    // std::process::exit(0);

    let default_outdir = std::env::current_dir().unwrap().join("recordings");

    std::fs::create_dir_all(&default_outdir).unwrap();

    let matches = App::new("Mouse and Keyboard Actions")
        .version("0.1.0")
        .author("darwin subramaniam")
        .about("Records and replay mouse and keyboard actions")
        .arg(
            Arg::with_name("status")
                .short("s")
                .value_name("status")
                .display_order(0)
                .possible_values(&["record", "r", "play", "p"])
                .help("Define Record or Play")
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("outdir")
                .default_value(default_outdir.to_str().unwrap())
                .short("o")
                .help("Path of directory to store the recording")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("recordFile")
                .short("f")
                .help("Define the record file")
                .required_ifs(&[("status", "p"), ("status", "play")])
                .takes_value(true),
        )
        .get_matches();

    initialize_engine(&matches);

    std::process::exit(0);
}

fn initialize_engine(matches: &clap::ArgMatches) {
    let status = matches.value_of("status").expect("No Status Value");
    let outdir = matches.value_of("outdir").unwrap();
    let record_file = matches.value_of("recordFile");

    let outdir_path = Path::new(outdir);

    if status == "play" || status == "p" {
        play::action(record_file.unwrap());
    }

    if status == "record" || status == "r" {
        println!("Start Recording");
        record::action(outdir_path);
    }
}
