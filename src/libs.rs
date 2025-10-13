use std::{fs::File, io::{BufRead, BufReader}};

pub fn readfile(file_name: &str) -> String {
  let file = File::open(file_name).expect("Could not open file");
  let reader = BufReader::new(file);

  let mut lines = Vec::<String>::new();

  for line in reader.lines() {
      match line {
          Ok(content) => lines.push(content),
          Err(e) => eprintln!("Error reading line: {}", e),
      }
  }

  lines.join("")
}