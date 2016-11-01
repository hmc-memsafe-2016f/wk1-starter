// Zach Hauser <zachary.hauser@pomona.edu>
// Submission for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;
use std::fmt::Debug;

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize
}

struct Node<T> {
    item : T,
    next: Option<Box<Node<T>>>
}

impl <T: Debug> SinglyLinkedList<T> {

    fn print_rec(node: &Node<T>) {
        println!("{:?}", node.item);
        match &node.next {
            &None => (),
            &Some(ref boxed_next) => SinglyLinkedList::print_rec(&*boxed_next)
        }
    }

    pub fn print(&self) {
        println!("=== Size: {} ===", self.size);
        match &self.head {
            &None => (),
            &Some(ref head) => SinglyLinkedList::print_rec(&*head)
        };
        println!("===")
    }

}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {

    fn new() -> Self {
        SinglyLinkedList { head: None, size: 0 }
    }

    fn push_front(&mut self, item: T) {
        self.size += 1;
        self.head = Some(Box::new(Node { item: item, next: self.head.take() }));
    }

    fn pop_front(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(mut boxed_head) => {
                self.size -= 1; 
                self.head = boxed_head.next.take();
                Some(boxed_head.item)
            }
        }
    }

    fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.item)
    }

    fn len(&self) -> usize {
        self.size
    }

    fn remove_first(&mut self, target: &T) -> Option<T> {
        None
    }

    
    fn reverse(&mut self) { 
        if self.head.is_none() {
            return
        }
        let mut node = *self.head.take().unwrap();
        let mut prev = None;
        while let Some(boxed_next) = node.next {
            let next = *boxed_next;
            node.next = prev;
            prev = Some(Box::new(node));
            node = next;
        }
        node.next = prev;
        self.head = Some(Box::new(node));
    }

}