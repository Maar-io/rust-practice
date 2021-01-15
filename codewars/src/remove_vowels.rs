fn disemvowel(s: &str) -> String {

  let vowels = String::from("aeiouAEIOU");
  let mut result: String = String::from("");

  for c in s.chars(){
    if !vowels.contains(c) {
      result.push(c);
    }
  }

  result
}

pub fn run(){
  println!("run disemvowel");
  let d = disemvowel("Mario");
  println!("disemvowel {}", d);

}

// ///////////////////////////////////////
// /// 
// fn disemvowel2(s: &str) -> String {
//   s.replace(&['A', 'E', 'I', 'O', 'U', 'a', 'e', 'i', 'o', 'u'][..], "")
//   }
  
  
//   ////////////////////////////
//   fn disemvowel3(s: &str) -> String {
//     let mut output = String::new();
    
//     for c in s.chars() {
//         match c {
//             'a'|'e'|'i'|'o'|'u'|'A'|'E'|'I'|'O'|'U' => (),
//             _ => output += &c.to_string(),
//         };
//     }
    
//     output
//   }
  
//   ///////////////////////////////
//   fn disemvowel4(s: &str) -> String {
//     let vowels = ['a', 'e', 'i', 'o', 'u', 'A', 'E', 'I', 'O', 'U'];
//     let mut res = String::from("");
//     for c in s.chars() {
//       if vowels.contains(&c) == false {
//         res.push(c);
//       }
//     }
//     res
//   }
  
#[cfg(test)]
mod tests {
  use super::disemvowel;
  
  #[test]
  fn example_test() {
      assert_eq!(disemvowel("This website is for losers LOL!"), "Ths wbst s fr lsrs LL!");
  }
}