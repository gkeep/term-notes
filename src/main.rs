use clap::{App, Arg};
use dirs;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

fn main() {
    let m = App::new("notes")
        .version("0.2.0")
        .author("gkeep")
        .about("Notes in your terminal!")
        .arg(
            Arg::with_name("local")
                .short("l")
                .long("local")
                .help("Read notes from current directory")
                .required(false)
                .takes_value(false),
        )
        .arg(
            Arg::with_name("body")
                .short("b")
                .long("with-body")
                .help("Print notes with body")
                .required(false)
                .takes_value(false),
        )
        .get_matches();

    /*
     * Notes location
     * --local to read from .notes subfolder in current directory
     * Default location for notes is /home/<user>/.local/Notes/notes.dat
     */
    let home_folder = dirs::home_dir().unwrap();
    let mut notes_location = Path::new(&home_folder).join(".local/Notes/notes.dat");

    /*
    ! move to add subcommand
    if !Path::exists(notes_location) {
        fs::create_dir(notes_location);
    }
    */
    if m.is_present("local") {
        // Local notes
        if Path::new(".notes/notes.dat").exists() {
            notes_location = PathBuf::from(".notes/notes.dat");
        } else {
            println!("No notes in this folder!");
            std::process::exit(0);
        }
    }

    println!("Your notes:");

    let note_file = notes_location.to_str().unwrap();

    if m.is_present("body") {
        print_notes(note_file, true);
    } else {
        // Print without body by default
        print_notes(note_file, false);
    }
}

fn print_notes(filename: &str, print_body: bool) {
    /*
     * print_notes
     * Prints notes from a specific file (filename)
     * print_body specifies whether to print note's body or not
     */
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut index = 1;

    for line in reader.lines() {
        let line = line.unwrap();

        if line != "" {
            if !line.contains("    ") {
                println!("  {}: {}", index, line);
                index += 1;
            } else if print_body {
                println!("   {}", line);
            }
        }
    }
}
