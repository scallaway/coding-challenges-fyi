use crate::flags::Flags;
use std::io;
use std::str::FromStr;
use std::{env::Args, fs};

// TODO: Handle reading from stdin
#[derive(Debug)]
pub struct Engine {
    pub file_name: Option<String>,
    // This is optional as there may not be any flags, in the event that we're
    // just displaying the default values
    pub flag: Option<Flags>,
}

impl Engine {
    pub fn new(args: Args) -> Engine {
        // We can always start from the second argument in the list as the
        // first is _always_ the program name
        let args = args.collect::<Vec<String>>()[1..].to_vec();

        Engine {
            file_name: get_file_name(&args),
            flag: if &args.len() > &1 {
                Some(Flags::from_str(&get_flag_from_args(&args)).expect("Could not parse flag"))
            } else {
                None
            },
        }
    }

    pub fn run(&self) {
        println!("{:?}", self);
        if let None = self.file_name {
            self.show_missing();
            return;
        }

        if self.flag.is_none() {
            self.run_default();
            return;
        }

        let file_name = &self.file_name.clone().unwrap();

        if self.flag.unwrap() == Flags::Bytes {
            println!(
                "{}",
                format!(
                    "{} {}",
                    fs::metadata(&file_name)
                        .expect("Couldn't find file specified")
                        .len() as usize,
                    &file_name
                )
            );
            return;
        }

        let file = fs::read_to_string(&file_name).expect("Couldn't read file into string");

        let statistic = match self.flag.unwrap() {
            Flags::Lines => file.lines().count(),
            Flags::Chars => file.chars().collect::<Vec<char>>().len(),
            Flags::Words => self.get_word_count(&file),
            Flags::LongestLine => self.get_longest_line(&file),
            // We shouldn't be able to get here since it's handled above in the if statement
            Flags::Bytes => return,
        };

        println!("{}", format!("{} {}", statistic, &file_name));
    }

    fn run_default(&self) {
        let file_name = &self.file_name.clone().unwrap();
        let file = fs::read_to_string(&file_name).expect("Couldn't read file into string");
        let lines = get_adjusted_line_count(&file);
        let words = self.get_word_count(&file);
        let bytes = file.as_bytes().len();

        println!(
            "{}",
            format!("{} {} {} {}", lines, words, bytes, &file_name)
        );
    }

    fn get_word_count(&self, file: &String) -> usize {
        let lines = file.lines();
        let line_count = lines.clone().count();
        lines.take(line_count - 1).fold(0, |acc, line| {
            acc + line.split_whitespace().collect::<Vec<&str>>().len()
        })
    }

    fn get_longest_line(&self, file: &String) -> usize {
        file.lines().fold(
            0,
            |acc, line| {
                if line.len() > acc {
                    line.len()
                } else {
                    acc
                }
            },
        )
    }

    fn show_missing(&self) {
        println!(
            "{}",
            format!(
                "No such file or directory {}",
                self.file_name.clone().unwrap()
            )
        );
    }
}

/// The bash implementation of wc doesn't count empty lines (this includes
/// lines that only contain the NULL character), so we have to account for
/// that here.
fn get_adjusted_line_count(file: &String) -> usize {
    let base_count = file.lines().count();

    if base_count > 1 {
        return base_count;
    }

    let line = file.lines().next().expect("Couldn't read line");

    return if line.trim_matches('\0').is_empty() {
        0
    } else {
        1
    };
}

fn get_file_name(args: &Vec<String>) -> Option<String> {
    if let Some(file_name) = args.iter().find(|&arg| !arg.starts_with("-")) {
        return Some(file_name.to_string());
    }

    let mut buffer = String::new();
    let result = io::stdin().read_line(&mut buffer);

    if result.is_err() {
        return None;
    }

    return Some(buffer);
}

/// Returns the first string that it find that does start with a hyphen
fn get_flag_from_args(args: &Vec<String>) -> String {
    return args
        .iter()
        .find(|&arg| arg.starts_with("-"))
        .unwrap()
        .to_owned();
}
