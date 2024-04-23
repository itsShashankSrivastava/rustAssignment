//Implement a function that checks whether a given string is a palindrome or not.


fn is_palindrome(input: &str) -> bool {
  let input = input.trim(); // Remove leading and trailing whitespace
  let length = input.len();
  for i in 0..length / 2 {
      if input[i..=i] != input[length - 1 - i..=length - 1 - i] {
          return false;
      }
  }
  true
}

fn main() {
  let mut input = String::new();
  println!("Enter a string: ");
  std::io::stdin().read_line(&mut input).expect("Failed to read line");

  if is_palindrome(&input) {
      println!("'{}' is a palindrome", input.trim());
  } else {
      println!("'{}' is not a palindrome", input.trim());
  }
}
