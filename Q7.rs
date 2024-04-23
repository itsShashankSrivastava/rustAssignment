//Implement a function that returns the kth smallest element in a given array.

fn partition(arr: &mut [i32], l: usize, r: usize) -> usize {
  let pivot = arr[r];
  let mut i = l;
  for j in l..r {
      if arr[j] <= pivot {
          arr.swap(i, j);
          i += 1;
      }
  }
  arr.swap(i, r);
  i
}

fn kth_smallest_element(arr: &mut [i32], l: usize, r: usize, k: usize) -> i32 {
  if k <= r - l + 1 && k > 0 {
      let ind = partition(arr, l, r);

      if ind - l == k - 1 {
          return arr[ind];
      }
      if ind - l > k - 1 {
          return kth_smallest_element(arr, l, ind - 1, k);
      }

      return kth_smallest_element(arr, ind + 1, r, k - ind + l - 1);
  }
  i32::MAX
}

fn main() {
  let mut arr = [12, 3, 5, 7, 4, 19, 26];
  let n = arr.len();
  let k = 3;
  println!("Kth Smallest element is {}", kth_smallest_element(&mut arr, 0, n - 1, k));
}
