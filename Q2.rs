//Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn binary_search_first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
  let mut low = 0;
  let mut high = arr.len();

  while low < high {
      let mid = low + (high - low) / 2;
      if arr[mid] < target {
          low = mid + 1;
      } else {
          high = mid;
      }
  }

  if low < arr.len() && arr[low] == target {
      Some(low)
  } else {
      None
  }
}

fn main() {
  let arr = vec![1, 2, 3, 4, 4, 4, 5, 6, 7];
  let target = 4;
  match binary_search_first_occurrence(&arr, target) {
      Some(index) => println!("First occurrence of {} is at index {}", target, index),
      None => println!("{} not found in the array", target),
  }
}
