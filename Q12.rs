//Find the maximum subarray sum in Rust

fn max_subarray_sum(arr: &[i32]) -> (i64, usize, usize) {
  let mut maxi = i64::MIN; // maximum sum
  let mut sum = 0;
  let mut start = 0;
  let mut ans_start = arr.len(); // Initialize with an invalid index
  let mut ans_end = arr.len();   // Initialize with an invalid index

  for (i, &num) in arr.iter().enumerate() {
      if sum == 0 {
          start = i; // starting index
      }

      sum += num as i64;

      if sum > maxi {
          maxi = sum;
          ans_start = start;
          ans_end = i;
      }

      // If sum < 0: discard the sum calculated
      if sum < 0 {
          sum = 0;
      }
  }

  (maxi, ans_start, ans_end)
}

fn main() {
  let arr = [-2, 1, -3, 4, -1, 2, 1, -5, 4];
  let (max_sum, start_index, end_index) = max_subarray_sum(&arr);
  println!("The maximum subarray sum is: {}", max_sum);
  if start_index < arr.len() && end_index < arr.len() {
      print!("The subarray is: [");
      for num in &arr[start_index..=end_index] {
          print!("{} ", num);
      }
      println!("]");
  } else {
      println!("No subarray found");
  }
}
