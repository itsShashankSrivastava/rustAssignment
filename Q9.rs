//Reverse a string in Rust

fn reverse_string(input: &str) -> String {
  let mut result = String::new();

  for c in input.chars().rev() {
      result.push(c);
  }

  result
}

fn main() {
  let input = "hello";
  let reversed = reverse_string(input);
  println!("Original string: {}", input);
  println!("Reversed string: {}", reversed);
}
