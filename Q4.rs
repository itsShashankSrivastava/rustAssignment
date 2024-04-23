//Implement a function that checks whether a given number is prime or not.

fn is_prime(n: i32) -> bool {
  if n <= 1 {
      return false;
  }
  for i in 2..=(n as f64).sqrt() as i32 {
      if n % i == 0 {
          return false;
      }
  }
  true
}

fn main() {
  let n = 20;
  let ans = is_prime(n);
  if n != 1 && ans {
      println!("Prime Number");
  } else {
      println!("Non Prime Number");
  }
}
