use clap::{Arg, App};
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
extern crate open;

fn main() {
    let m = App::new("notes")
                    .version("0.1")
                    .author("gkeep")
                    .about("Notes in your terminal!")
                    .arg(Arg::with_name("print")
                        .short("p")
                        .long("print")
                        .help("Print notes with or without body")
                        .possible_values(&["head", "body"])
                        .required(false)
                        .takes_value(true))
                    .arg(Arg::with_name("edit")
                        .short("e")
                        .long("edit")
                        .help("Edit notes")
                        .takes_value(false))
                    .get_matches();

    if m.is_present("print") {
        let file = "Notes/1.txt";
        match m.value_of("print").unwrap() {
            "head" => {
                println!("Your notes:");
                print_notes(file, false);
            }
            "body" => {
                println!("Your notes:");
                print_notes(file, true);
            }
            _ => {
                println!("ERROR: Couldn't read notes!");
            }
        }
    }
    else if m.is_present("edit") {
        match env::var("EDITOR") {
            Ok(val) => {
                open::with("Notes/1.txt", val).expect("Couldn't open the editor");
            },
            Err(e) => println!("Couldn't open editor ($EDITOR env variable): {}", e),
        }
    }
}

fn print_notes(filename: &str, print_body: bool) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut note_index = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if !line.contains("    ") && line != "" {
            note_index += 1;
            println!("    {}: {}", note_index, line);
        } else {
            if print_body {
                println!("    {}", line);
            }
        }
    }
}
