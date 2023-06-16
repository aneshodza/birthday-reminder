use std::fs::File;
use std::path::{Path, PathBuf};
use std::io;
use std::process::exit;

fn main() {
    let file_dir = dirs::home_dir().unwrap().join(".birthdays.txt");
    if check_for_note_file(&file_dir) {
        println!("File exists");
    } else {
        initialize_note_file(&file_dir);
    }
}

fn check_for_note_file(dir: &PathBuf) -> bool {
    return Path::new(dir).exists();
}

fn initialize_note_file(dir: &PathBuf) {
    println!("Looks like you didn't initialize your birthday init file yet.");
    println!("Would you like to initialize it now? (y/n)");
    if read_y_n_input() {
        println!("Initializing file...");
        File::create(dir).expect("Unable to create file");
        println!("File initialized!");
    } else {
        println!("Alright. Bye!");
        exit(69);
    }
}

fn read_y_n_input() -> bool {
    let mut stdin = String::new();
    io::stdin().read_line(&mut stdin).ok();

    return match stdin.as_str() {
        "y\n" => true,
        "n\n" => false,
        _ => {
            println!("Please enter y or n.");
            return read_y_n_input();
        }
    }
}
