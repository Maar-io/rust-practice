fn main() {
    println!("Hello, world!");
}

use std::collections::HashMap;

pub trait PriorityQueue<Element> {
    /// create a new priority queue.
    fn new() -> Self;
    /// check whether the queue has no elements.
    fn is_empty(&self) -> bool;
    /// returns the highest-priority element but does not modify the queue.
    fn peek(&self) -> Option<Element>;
    /// add an element to the queue with an associated priority.
    fn insert(&mut self, element: Element, priority: u64);
    /// remove the element from the queue that has the highest priority, and return it.
    fn pop(&mut self) -> Option<Element>;
}

pub struct PriorityQueueImpl(HashMap<Vec<u8>, Vec<u8>>);

// Do not modify anything above ^^^

impl PriorityQueue<Vec<u8>> for PriorityQueueImpl {
    // TODO: finish the implementation

    fn new() -> Self {
        unimplemented!()
    }

    fn is_empty(&self) -> bool {
        unimplemented!()
    }

    fn peek(&self) -> Option<Vec<u8>> {
        unimplemented!()
    }

    fn insert(&mut self, element: Vec<u8>, priority: u64) {
        unimplemented!()
    }

    fn pop(&mut self) -> Option<Vec<u8>> {
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut queue = PriorityQueueImpl::new();
        assert!(queue.is_empty());

        queue.insert(vec![0], 5);
        assert!(!queue.is_empty());
        assert_eq!(queue.peek(), Some(vec![0]));

        queue.insert(vec![1], 10);
        queue.insert(vec![2], 3);
        queue.insert(vec![3], 4);
        queue.insert(vec![4], 6);

        assert_eq!(queue.pop(), Some(vec![1]));
        assert_eq!(queue.pop(), Some(vec![4]));
        assert_eq!(queue.pop(), Some(vec![0]));
        assert_eq!(queue.pop(), Some(vec![3]));
        assert_eq!(queue.pop(), Some(vec![2]));

        assert!(queue.is_empty());
    }

    // TODO: add more tests as appropriate
}