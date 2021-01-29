// Given a random non-negative number, you have to return the digits of this number within an array in reverse order.
// 
// Example:
// 348597 => [7,9,5,8,4,3]



fn digitize(mut n: u64) -> Vec<u8> {
  // your code here
  let mut result: Vec<u8> = Vec::new();

  loop{
    let x: u8 = (n % 10) as u8;
    result.push(x);
    n = n / 10;
    if n == 0{
      break;
    }
  }
  result
}

// fn digitize(n: u64) -> Vec<u8> {
//   n
//       .to_string()
//       .chars()
//       .map(|c| c.to_digit(10).unwrap() as u8)
//       .rev()
//       .collect::<Vec<u8>>()
// }


pub fn run(){
  println!("run digitize");
  let inv = digitize(0);
  println!("Hi {:?}", inv);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed() {
        assert_eq!(digitize(35231), vec![1,3,2,5,3]);
    }
    #[test]
    fn test_small() {
      assert_eq!(digitize(2), vec![2]);
    }
    
    #[test]
    fn test_zero() {
      assert_eq!(digitize(0), vec![0]);
  }
}