fn is_token_separator(c: char) -> bool {
  match c {
    ' ' | ':' | ';' | '[' | ']' | '{' | '}' | '(' | ')' | '<' | '>' | ',' => true,
    _ => false,
  }
}

pub fn to_token_chunks(source_string: &str) -> Vec<String> {
  let mut char_chunks = Vec::<String>::new();
  let mut chunk_buffer = Vec::<char>::new();
  let mut chars = source_string.chars().peekable();
  let mut char: char;
  while chars.peek() != None {
    char = chars.next().unwrap();
    if is_token_separator(char) {
      if !chunk_buffer.is_empty() {
        char_chunks.push(chunk_buffer.iter().collect());
        chunk_buffer.clear();
      }
      char_chunks.push(char.to_string());
    } else {
      chunk_buffer.push(char);
    }
  }

  char_chunks
}