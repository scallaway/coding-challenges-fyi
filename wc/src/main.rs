use std::{env, time::Instant, todo};

fn main() {
    let now = Instant::now();

    println!("{}", parse_args(get_args()));

    // Only print the time taken in debug mode
    if cfg!(debug_assertions) {
        println!("Took: {}ns", now.elapsed().as_nanos());
    }
}

fn get_args() -> Vec<String> {
    return env::args().collect::<Vec<String>>()[1..].to_vec();
}

fn parse_args(args: Vec<String>) -> String {
    let arg = args.get(0);
    let file_name = args.get(1);

    // In the future, this will show the help screen
    if arg.is_none() {
        panic!("No arguments provided");
    }

    // If there aren't any flags, then we return the default option
    if file_name.is_none() {
        let (file_as_string, file_as_bytes) = read_file_in_string_and_bytes(arg.unwrap());
        return default_output(&file_as_string, &file_as_bytes, arg.unwrap().to_owned());
    }

    let file_name = file_name.unwrap();

    let file_as_string = std::fs::read_to_string(file_name).expect("Unable to read file");
    let file_as_bytes = std::fs::read(file_name).expect("Unable to read file");

    format!(
        "{} {}",
        match arg.unwrap().as_str() {
            "-c" => get_byte_count(&file_as_bytes),
            "-l" => get_line_count(&file_as_string),
            "-w" => get_word_count(&file_as_string),
            "-m" => get_char_count(&file_as_string),
            _ => todo!("Unrecognised command"),
        },
        file_name
    )
}

// Read the file in both formats to avoid having to read the file twice with
// the defaults
fn read_file_in_string_and_bytes(file_name: &str) -> (String, Vec<u8>) {
    let file_as_string = std::fs::read_to_string(file_name).expect("Unable to read file");
    let file_as_bytes = std::fs::read(file_name).expect("Unable to read file");

    (file_as_string, file_as_bytes)
}

fn default_output(file_as_string: &String, file_as_bytes: &Vec<u8>, file_name: String) -> String {
    return format!(
        "{} {} {} {}",
        get_line_count(file_as_string),
        get_word_count(file_as_string),
        get_byte_count(file_as_bytes),
        file_name
    );
}

fn get_byte_count(file: &Vec<u8>) -> usize {
    file.len()
}

fn get_line_count(file: &String) -> usize {
    file.lines().count()
}

fn get_word_count(file: &String) -> usize {
    file.lines().fold(0, |acc, line| {
        acc + line.split_whitespace().collect::<Vec<&str>>().len()
    })
}

fn get_char_count(file: &String) -> usize {
    file.chars().collect::<Vec<char>>().len()
}
