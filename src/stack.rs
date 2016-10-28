// Zach Hauser <zachary.hauser@pomona.edu>
// Submission for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;
use std::mem;

pub struct SinglyLinkedList<T> {
    head: Option<Node<T>>,
    size: usize
}

struct Node<T> {
    item : T,
    next: Option<Box<Node<T>>>,
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {

    fn new() -> Self {
        SinglyLinkedList { head: None, size: 0 }
    }

    fn push_front(&mut self, item: T) {
        self.size += 1;
        let mut node = Node { item: item, next: None };
        if self.head.is_some() {
            node.next = self.head.take().map(Box::new);
            self.head = Some(node)
        } else {
            self.head = Some(node)
        };
    }

    fn pop_front(&mut self) -> Option<T> {
        let (next, result) = match self.head.take() {
            None => (None, None),
            Some(head) => {
                self.size += 1; 
                (head.next.map(|next| *next), Some(head.item))
            }
        };
        self.head = next;
        result 
    }

    fn peek_front(&self) -> Option<&T> {
        match self.head {
            None => None,
            Some(ref head) => Some(&head.item)
        }
    }

    fn len(&self) -> usize {
        self.size
    }

    fn reverse(&mut self) { 
        
    }

}