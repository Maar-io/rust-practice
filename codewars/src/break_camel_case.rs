// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

fn break_camel_case(s: &str) -> String {
    let mut s1 = String::new();
    for c in s.chars() {
        if  c.is_uppercase() {
            s1.push(' ');
        }
    s1.push(c);
    }   
    s1
}

// fn first_letter_to_lower_case (s1: String) -> String {
//     let mut c = s1.chars();
//     match c.next() {
//       None => String::new(),
//       Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
//     }
//   }

pub fn run(){
    println!("run break_camel_case");
    let broken = break_camel_case("camelCaseTo");
    println!("Broken {:?}", broken);
  }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(break_camel_case("camelCasingTest"), "camel Casing Test");
        assert_eq!(break_camel_case("camelCasing"), "camel Casing");
    }
}