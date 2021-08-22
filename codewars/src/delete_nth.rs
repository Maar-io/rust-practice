use std::collections::HashMap;

fn delete_nth(lst: &[u8], n: usize) -> Vec<u8> {
    let mut collect = HashMap::new();
    let mut counter;
    let mut res: Vec<u8> = Vec::new();
    for item in lst {
        // limit = match collect.get(&item){
        //     Some(i) => {},
        //     None => {collect.insert(item, 1); 1}
        // };
     *collect.entry(item).or_insert(0) += 1;
        counter = collect.get(item).unwrap();
        if counter <= &n {
            res.push(*item);
        };
    }
    res
}


pub fn run(){
    println!("run delete_nth");
    let res = delete_nth(&[20,37,20,21], 1);
    println!
    ("new vec {:?}", res);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(delete_nth(&[20,37,20,21], 1), vec![20,37,21]);
        assert_eq!(delete_nth(&[1,1,3,3,7,2,2,2,2], 3), vec![1, 1, 3, 3, 7, 2, 2, 2]);
    }
}