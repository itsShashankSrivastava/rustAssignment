//Implement a function that finds the longest common prefix of a given set of strings.

fn longest_common_prefix(arr: &[&str]) -> String {
  let mut result = arr[0].to_string();

  for i in 1..arr.len() {
      while !arr[i].starts_with(&result) {
          result.pop();
          if result.is_empty() {
              return "-1".to_string();
          }
      }
  }

  result
}

fn main() {
  let input = ["flower", "flow", "flight"];;
  println!("The longest Common Prefix is: {}", longest_common_prefix(&input));
}
