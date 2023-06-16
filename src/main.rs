use std::fs::{File, read_to_string};
use std::path::{Path, PathBuf};
use std::io;
use std::process::exit;

use chrono::{NaiveDate, Local};

fn main() {
    let file_dir = dirs::home_dir().unwrap().join(".birthday_reminder/.birthdays.txt");
    if check_for_note_file(&file_dir) {
        print_birthdays(&file_dir);
    } else {
        initialize_note_file(&file_dir);
    }
}

fn check_for_note_file(dir: &PathBuf) -> bool {
    return Path::new(dir).exists();
}

fn print_birthdays(dir: &PathBuf) {
    let contents = read_to_string(&dir).expect("Unable to read file");
    if contents.is_empty() {
        println!("You dont have any birthdays yet!");
        exit(0);
    }
    let mut lines: Vec<&str> = contents.split("\n").collect();
    lines = lines[..lines.len() - 1].to_vec();
    search_soon_birthday(lines);
}

fn search_soon_birthday(data: Vec<&str>) {
    let today = Local::now().naive_local().date();
    let mut any_birthday = false;
    for row in data.iter() {
        let result = parse_row(row, today);
        if result {
            any_birthday = true;
        }
    }
    if !any_birthday {
        println!("No birthdays in the next 14 days!");
    }
}

fn parse_row(row: &str, today: NaiveDate) -> bool {
    let split_row: Vec<&str> = row.split("=").collect();
    let name = split_row[0];
    let date_of_birth = NaiveDate::parse_from_str(split_row[1], "%Y.%m.%d").expect("Invalid date format");
    let days_old = today.signed_duration_since(date_of_birth).num_days();
    let until_birthday = days_until_bithday(days_old);
    if until_birthday < 14 && until_birthday > 0 {
        println!("{} has birthday in {} days!", name, until_birthday);
        return true;
    } else if until_birthday == 0 {
        println!("{} turns {} today!", name, days_old / 365);
        return true;
    }
    false
}

fn days_until_bithday(days: i64) -> i64 {
    return days % 365;
}

fn initialize_note_file(dir: &PathBuf) {
    println!("Looks like you didn't initialize your birthday init file yet.");
    println!("Would you like to initialize it now? (y/n)");
    if read_y_n_input() {
        println!("Initializing file...");
        File::create(dir).expect("Unable to create file");
        println!("File initialized!");
        main();
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
