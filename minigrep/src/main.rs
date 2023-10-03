use std::{env, fs, process};
use minigrep;

fn main() {
    let arguments: Vec<String> = env::args().collect();

    let config = minigrep::Config::new(&arguments);

    let file_contents = minigrep::search_file(&config);

    match file_contents {
        Ok(lines) => {
            for line in lines {
                println!("{}", line);
            }
        }
        Err(e) => {
            eprintln!("{}", e); // Print to stderr
            process::exit(1);
        }
    }
}
