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

    print!("New note: ");
    // Print user input on the same line
    let _ = io::stdout().flush();

    let mut note_title = String::new();
    io::stdin().read_line(&mut note_title).expect("Could not read note title!"); // Read note title

    // If input is empty, return
    if note_title.len() <= 1 {
        println!("No note title provided! Exiting...");
        return;
    }

    print!("Body: ");
    let _ = io::stdout().flush();

    let mut note_body_inp = String::new().to_owned();
    io::stdin().read_line(&mut note_body_inp).expect("Could not read note body!");

    // If input is empty, return
    if note_body_inp.len() <= 1 {
        return;
    }

    let mut note_body = String::from("    ").to_owned(); // Body starts with 4 spaces
    note_body.push_str(&note_body_inp); // Combine input with line offset
    file.write_all(note_body.as_bytes()).expect("Could not write note body to the notes.dat file!"); // Write it
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