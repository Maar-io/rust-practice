fn solution(phrase: &str) -> String {
  phrase.chars().rev().collect()
}

pub fn run(){
  let inv = solution("Heloo");
  println!("Hi {}", inv);
}



#[cfg(test)]
extern crate rand;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(solution("world"), "dlrow");
        assert_eq!(solution("hello"), "olleh");
        assert_eq!(solution(""), "");
        assert_eq!(solution("h"), "h");
    }
}