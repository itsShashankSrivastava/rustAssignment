//Given a string of words, implement a function that returns the shortest word in the string.


fn find_shortest_word(s: &str) -> Option<&str> {
  s.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
  let input = "This is a test string";
  match find_shortest_word(input) {
      Some(shortest_word) => println!("Shortest word: {}", shortest_word),
      None => println!("No words found in the input"),
  }
}