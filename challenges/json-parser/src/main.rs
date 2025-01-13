use crate::parser::Parser;
use env_logger;
use log::{debug, error, info};
use std::{env, process};

mod lexer;
mod parser;

fn main() {
    env_logger::init();

    let Some(path) = env::args().nth(1) else {
        return error!("json-valiator requires a path to a file to validate");
    };

    let Ok(file) = std::fs::read_to_string(path) else {
        return error!("Failed to read the file from the path provided. Does it actually exist?",);
    };

    let json_parser = Parser::new(&file);

    let result = json_parser.parse();
    debug!("Our result: {:?}", result.clone());
    // Use serde_json whilst we're still testing
    debug!("Their result: {:?}", serde_json::from_str::<String>(&file));

    if result.is_err() {
        error!("{}", result.unwrap_err());
        process::exit(1);
    }

    info!("Provided file is valid JSON");
}
