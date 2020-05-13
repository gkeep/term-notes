use clap::{App, Arg};
use std::io::{BufRead, BufReader};
use std::{env, fs, fs::File};
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
        .arg(
            Arg::with_name("edit")
                .short("e")
                .long("edit")
                .help("Edit notes")
                .takes_value(false),
        )
        .get_matches();

    // Custom location of notes
    let notes_custom_location;
    match env::var("notes_custom_location") {
        Ok(val) => {
            notes_custom_location = val;
        }
        _ => {
            let home_folder = env::var("HOME").unwrap();
            notes_custom_location = format!("{}/Notes", home_folder);
        }
    }

    if m.is_present("print") {
        let notes_folder = fs::read_dir(notes_custom_location).unwrap();
        let mut note_index = 1;

        println!("Your notes:");

        for file in notes_folder {
            let file_loc = file.unwrap().path().to_string_lossy().to_string();

            match m.value_of("print").unwrap() {
                "head" => {
                    print_notes(file_loc, false, note_index);
                }
                "body" => {
                    print_notes(file_loc, true, note_index);
                }
                _ => {
                    println!("ERROR: Couldn't read notes!");
                }
            }

            note_index += 1;
        }
    } else if m.is_present("edit") {
        match env::var("EDITOR") {
            Ok(val) => {
                open::with("Notes/1.txt", val).expect("Couldn't open the editor");
            }
            Err(e) => println!("Couldn't open editor ($EDITOR env variable): {}", e),
        }
    }
}

fn print_notes(filename: String, print_body: bool, index: i32) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        if !line.contains("    ") && line != "" {
            println!("    {}: {}", index, line);
        } else {
            if print_body {
                println!("    {}", line);
            }
        }
    }
}
