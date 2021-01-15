// This kata is from check py.checkio.org

// You are given an array with positive numbers and a non-negative number N. 
// You should find the N-th power of the element in the array with the index N. 
// If N is outside of the array, then return -1. Don't forget that the first element has the index 0.

// Let's look at a few examples:

// array = [1, 2, 3, 4] and N = 2, then the result is 3^2 == 9;
// array = [1, 2, 3] and N = 3, but N is outside of the array, so the result is -1.

fn index(nums: &[u64], n: usize) -> Option<u64> {
  if nums.len() > n {
    return Some(nums[n].pow(n as u32));
  }
  None
}


// fn index(nums: &[u64], n: usize) -> Option<u64> {
//   nums.get(n).map(|x| x.pow(n as u32))
// }

// fn index(nums: &[u64], n: usize) -> Option<u64> {
//   nums.get(n).and_then(|x| Some(x.pow(n as u32)))
// }

// fn index(nums: &[u64], n: usize) -> Option<u64> {
//   match nums.get(n) {
//       Some(i) => { return Some(i.pow(n as u32)); }
//       None => { return None; }
//   }
// }

pub fn run(){
  println!("run index2");
  index(&[1, 1], 0);
}

#[cfg(test)]
mod tests {
    use super::index;

    #[test]
    fn sample_tests() {
        assert_eq!(index(&[1, 2, 3, 4], 2), Some(9), "Failed on the first sample test");
        assert_eq!(index(&[1, 3, 10, 100], 3), Some(1000000), "Failed on the second sample test");
        assert_eq!(index(&[1, 2, 3, 4], 69), None, "Failed on the third sample test");
        assert_eq!(index(&[1, 1], 0), Some(1), "Failed on the 4 sample test");
        assert_eq!(index(&[5, 9], 0), Some(1), "Failed on the 5 sample test");
    }
}
