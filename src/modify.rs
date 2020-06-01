use std::path::Path;
use std::fs::{OpenOptions, File, self};
use std::io::{BufRead, BufReader};
use std::io;
use std::io::prelude::*;

pub fn add_note(file_location: &Path) {
    // Check if file exists, create if it does not
    create_file(file_location);

    let mut file = OpenOptions::new()
        .append(true)
        .open(file_location)
        .unwrap();

    print!("Note title: ");

    let mut note_title = String::new();
    // Print user input on the same line
    let _ = io::stdout().flush();

    io::stdin().read_line(&mut note_title).expect("Could not read note title!");

    file.write_all(note_title.as_bytes()).expect("Could not write note title to the notes.dat file!");
    // TODO: Add creation of note body
}

pub fn delete_note(file_location: &Path) {
    // ! rewrite this shit because file is not opened as modifiable
    // ! also, think about how the hell is it going to delete the notes
    // * atleast it compiles lmao
    let mut note_indexes = Vec::new();
    let mut user_indexed_notes = Vec::new();
    let mut user_index = 1;

    let file = File::open(file_location).unwrap();
    let reader = BufReader::new(file);

    // Get indexes of notes
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        if line != "" {
            if !line.contains("    ") {
                note_indexes.push(index);
                user_indexed_notes.push(user_index);
                user_index += 1;
            }
        }
    }

    for x in &note_indexes {
        println!("{} - note index", x);
    }

    for y in &user_indexed_notes {
        println!("{} - what user sees as notes indexes", y);
    }

    // Prompt user to choose a note they want deleted
}

fn create_file(file_location: &Path) {
    if !Path::exists(file_location) {
        // Create .notes folder and notes.dat
        fs::create_dir(file_location.parent().unwrap()).expect("Could not create .notes folder!");
        File::create(file_location).expect("Could not create notes.dat file!");
    }
}