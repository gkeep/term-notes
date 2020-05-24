use clap::{App, Arg};
use std::io::{BufRead, BufReader};
use std::{env, fs, fs::File};
use std::path::Path;
use std::ffi::OsStr;
extern crate open;

fn main() {
    let m = App::new("notes")
        .version("0.1")
        .author("gkeep")
        .about("Notes in your terminal!")
        .arg(
            Arg::with_name("print")
                .short("p")
                .long("print")
                .help("Print notes with or without body")
                .possible_values(&["head", "body"])
                .required(false)
                .takes_value(true),
        )
        .get_matches();

    // Custom location of notes
    let notes_custom_location;
    match env::var("NOTES_CUSTOM_LOCATION") {
        Ok(val) => {
            notes_custom_location = val;
        }
        _ => {
            let home_folder = env::var("HOME").unwrap();
            notes_custom_location = format!("{}/Notes", home_folder);
        }
    }

    let notes_folder = fs::read_dir(notes_custom_location).unwrap();

    if m.is_present("print") {
        let mut note_index = 1;

        println!("Your notes:");

        for file in notes_folder {
            let file_loc = file.unwrap().path().to_string_lossy().to_string();

            if !(Path::new(&file_loc).extension().and_then(OsStr::to_str) == Some("txt")) {
                continue;
            }

            print!("  {}:", note_index);

            match m.value_of("print").unwrap() {
                "head" => {
                    print_notes(file_loc, false);
                }
                "body" => {
                    print_notes(file_loc, true);
                }
                _ => {
                    println!("ERROR: Couldn't read notes!");
                }
            }

            note_index += 1;
        }
    }
}

fn print_notes(filename: String, print_body: bool) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line != "" {
            if !line.contains("    ") {
                if index > 1 {
                    println!("      {}", line);
                    continue;
                }
                println!("  {}", line);
            }
            else if print_body {
                println!("     {}", line);
            }
        }
    }
}
