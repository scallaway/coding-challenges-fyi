use crate::flags::Flags;
use core::str::FromStr;
use std::{env::Args, fs, io, io::Read};

#[derive(Debug)]
pub struct Engine {
    pub file_name: Option<String>,
    /// The input type to read from, whether that's a file name or stdin
    pub content: String,
    /// Any flags that have been parsed as valid to the program
    pub flags: Vec<Flags>,
}

// The beating heart of the program, where all the computation takes place
impl Engine {
    pub fn new(args: Args) -> io::Result<Engine> {
        // We can always start from the second argument in the list as the
        // first is _always_ the program name
        // TODO: Panic in the event the above isn't true
        let args = args.collect::<Vec<String>>()[1..].to_vec();

        let flags = get_flags_from_args(&args)
            .into_iter()
            .filter_map(|flag| {
                if let Ok(parsed_flag) = Flags::from_str(flag) {
                    Some(parsed_flag)
                } else {
                    None
                }
            })
            .collect();

        let file_name = get_file_name_from_args(&args);

        Ok(Engine {
            file_name: file_name.clone(),
            content: if let Some(file_name) = file_name {
                fs::read_to_string(&file_name).expect("Unable to find file")
            } else {
                // We only want to try to read from stdin when we haven't been given a file to try
                // and read from - otherwise we'll hang indefinitely.
                let mut buffer = String::new();
                let mut stdin = io::stdin();

                while let Ok(bytes_read) = stdin.read_to_string(&mut buffer) {
                    if bytes_read == 0 {
                        break;
                    }
                }

                buffer
            },
            flags,
        })
    }

    pub fn run(&self) {
        if self.flags.len() == 0 {
            return self.run_default();
        }

        println!(
            "{}",
            format!(
                "{} {}",
                &self.get_statistics_line(),
                self.file_name.clone().unwrap_or("".to_string())
            )
        );
    }

    fn run_default(&self) {
        let lines = get_adjusted_line_count(&self.content);
        let words = self.get_word_count();
        let bytes = self.content.as_bytes().len();

        println!(
            "{}",
            format!(
                "{} {} {} {}",
                lines,
                words,
                bytes,
                // We don't want to try and show the file name if we don't have one
                &self.file_name.clone().unwrap_or("".to_string()),
            )
        );
    }

    fn get_word_count(&self) -> usize {
        let lines = &self.content.lines();
        let line_count = lines.clone().count();
        lines.clone().take(line_count - 1).fold(0, |acc, line| {
            acc + line.split_whitespace().collect::<Vec<&str>>().len()
        })
    }

    fn get_longest_line(&self) -> usize {
        self.content.lines().fold(0, |acc, line| {
            if line.len() > acc {
                line.len()
            } else {
                acc
            }
        })
    }

    fn get_statistics_line(&self) -> String {
        self.flags
            .iter()
            .map(|flag| {
                return match flag {
                    Flags::Bytes => self.content.as_bytes().len(),
                    Flags::Lines => self.content.lines().count(),
                    Flags::Chars => {
                        self.content.chars().collect::<Vec<char>>().len()
                    }
                    Flags::Words => self.get_word_count(),
                    Flags::LongestLine => self.get_longest_line(),
                };
            })
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(" ")
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

fn get_file_name_from_args(args: &Vec<String>) -> Option<String> {
    return args.iter().find(|&arg| !arg.starts_with("-")).cloned();
}

fn get_flags_from_args(args: &Vec<String>) -> Vec<&String> {
    args.iter().filter(|&arg| arg.starts_with("-")).collect()
}
