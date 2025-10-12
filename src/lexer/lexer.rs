use std::{fs::File, io::{BufRead, BufReader}};

fn lexer(file_name: &str) {
  let file = File::open(file_name).expect("Could not open file");
  let reader = BufReader::new(file);

  for line in reader.lines() {
      match line {
          Ok(content) => println!("{}", content),
          Err(e) => eprintln!("Error reading line: {}", e),
      }
  }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let filename = "./src/lexer/test/sample.txt";
        lexer(filename);
    }
}
