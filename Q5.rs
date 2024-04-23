//Given a sorted array of integers, implement a function that returns the median of the array.

fn find_median(nums: &[i32]) -> f64 {
  let len = nums.len();
  let mid = len / 2;
  if len % 2 == 0 {
      (nums[mid - 1] as f64 + nums[mid] as f64) / 2.0
  } else {
      nums[mid] as f64
  }
}

fn main() {
  let nums1 = [1, 2, 3];
  println!("Median of {:?}: {}", nums1, find_median(&nums1));

  let nums2 = [1, 2, 3, 4];
  println!("Median of {:?}: {}", nums2, find_median(&nums2));
}
