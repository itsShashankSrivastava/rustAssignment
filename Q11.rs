//Merge two sorted arrays in Rust

fn merge(arr1: &mut [i64], arr2: &mut [i64], n: usize, m: usize) {
  let mut left = n as i64 - 1;
  let mut right = 0;

  while left >= 0 && right < m as i64 {
      if arr1[left as usize] > arr2[right as usize] {
          arr1.swap(left as usize, right as usize);
          left -= 1;
          right += 1;
      } else {
          break;
      }
  }

  arr1.sort();
  arr2.sort();
}

fn main() {
  let mut arr1 = [1, 4, 8, 10];
  let mut arr2 = [2, 3, 9];
  let n = 4;
  let m = 3;

  merge(&mut arr1, &mut arr2, n, m);

  println!("The merged arrays are:");
  print!("arr1[] = ");
  for i in 0..n {
      print!("{} ", arr1[i]);
  }
  print!("\narr2[] = ");
  for i in 0..m {
      print!("{} ", arr2[i]);
  }
  println!();
}
