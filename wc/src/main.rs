use std::{env, time::Instant, todo};

fn main() {
    let now = Instant::now();
    let args = get_args();

    println!("{}", parse_args(args));
    println!("Took: {}ns", now.elapsed().as_nanos());
}

fn get_args() -> Vec<String> {
    return env::args().collect::<Vec<String>>()[1..].to_vec();
}

fn parse_args(args: Vec<String>) -> i64 {
    if args.len() == 1 {
        todo!("Must supply a flag");
    }

    if args[0] == "-c" {
        return get_byte_count(args.get(1).unwrap());
    }

    return -1;
}

fn get_byte_count(file_name: &str) -> i64 {
    let file = std::fs::read_to_string(file_name).expect("Unable to read file");
    return file.len() as i64;
}
