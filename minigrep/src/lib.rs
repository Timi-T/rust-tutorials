use std::{fs, process};

pub struct Config {
  pub filename: String,
  pub query: String,
}

impl Config {
  pub fn new(arguments: &Vec<String>) -> Config {
      if arguments.len() < 3 {
        eprintln!("Please provide filename and search string"); // Print to stderr
        process::exit(1);
      }
      let filename: String = arguments[1].clone();
      let query: String = arguments[2].clone();

      println!("Searching for lines with [{}] in file [{}]", query, filename);

      Config {
          filename,
          query
      }
  }
}

pub fn search_file(config: &Config) -> Result<Vec<String>, String> {
  let file_contents = fs::read_to_string(&config.filename);
  match file_contents {
      Ok(file_contents) => {
          /* let lines = file_contents.lines().filter(|line| line.contains(
            &config.query
          ));
          return Ok(lines.collect()); */
          let mut lines: Vec<String> = vec![];
          for line in file_contents.lines() {
              if line.contains(&config.query) {
                  lines.push(line.to_string());
              }
          }
          return Ok(lines);
      }
      Err(e) => {
          return Err(String::from("Error reading from file"));
      }
  }
}

#[cfg(test)]
mod tests {
    use crate::{Config, search_file};

  #[test]
  fn test_search_file_one() {
    let config = Config::new(&vec![
      String::from(""),
      String::from("poem.txt"),
      String::from("nobody")
    ]);

    let expected = vec!["I'm nobody! Who are you?", "Are you nobody, too?"];

    match search_file(&config) {
        Ok(res) => {
          assert_eq!(res, expected);
        }
        Err(e) => {
          panic!("{}", e);
        }
    }
  }

  #[test]
  fn test_search_file_two() {
    let config = Config::new(&vec![
      String::from(""),
      String::from("poem.txt"),
      String::from("this line can never exist simply because it doesn't exist")
    ]);

    let expected: Vec<&str>= vec![];

    match search_file(&config) {
        Ok(res) => {
          assert_eq!(res, expected);
        }
        Err(e) => {
          panic!("{}", e);
        }
    }
  }

  #[test]
  fn test_search_file_three() {
    let config = Config::new(&vec![
      String::from(""),
      String::from("none.txt"),
      String::from("this line can never exist simply because it doesn't exist")
    ]);

    let expected: Vec<&str>= vec![];

    match search_file(&config) {
        Ok(res) => {
          panic!();
        }
        Err(e) => {
          assert_eq!(e, "Error reading from file");
        }
    }
  }
}