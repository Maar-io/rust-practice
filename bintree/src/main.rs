
struct Node {
  value: i32,
  left: Option<Box<Node>>,
  right: Option<Box<Node>>,
}


impl Node{
  // new node
  fn new(val: i32) -> Node {
      Node {value: val, left: None, right: None}
  }

  // insert new node
  pub fn insert(&mut self, n: Node){
    if n.value < self.value{
      match self.left {
        None => {
          println!("INSERT {} left {}",self.value, n.value);
          let n = Node::new(n.value);
          self.left = Some(Box::new(n));
        },
        Some(ref mut l) => l.insert(n),
      }
    }
    else{
      match self.right {
        None => {
          println!("INSERT {} right {}",self.value, n.value);
          let n = Node::new(n.value);
          self.right = Some(Box::new(n));
        },
        Some(ref mut r) => r.insert(n),
      }
    }
  }

  // traverse the tree
  fn traverse(&self){
    match self.left {
      None => {},
      Some(ref l) => l.traverse(),
    }
    println!("{}", self.value);
    match self.right {
      Some(ref r) => r.traverse(),
      None => {}
    }
  }
}

fn main() {

    let mut root = Node::new(5);
    root.insert(Node::new(4));
    root.insert(Node::new(7));
    root.insert(Node::new(3));
    root.insert(Node::new(2));
    root.insert(Node::new(12));
    root.insert(Node::new(12));
    root.insert(Node::new(6));

    root.traverse();
}
