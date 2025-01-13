mod engine;
mod flags;

use engine::Engine;
use std::{env, time::Instant};

/// wc - print the number of lines, words, and bytes in files
///
/// This solves the Coding Challenges "Build Your Own wc Tool" challenge found here:
/// https://codingchallenges.fyi/challenges/challenge-wc
fn main() {
    let now = Instant::now();

    let wc = Engine::new(env::args()).expect("Could not initialise engine");

    wc.run();

    // Only print the time taken in debug mode
    if cfg!(debug_assertions) {
        let elapsed = now.elapsed();
        println!("Took: {}ms ({}ns)", elapsed.as_millis(), elapsed.as_nanos());
    }
}
