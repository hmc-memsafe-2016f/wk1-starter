// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use std::mem;

use Stack;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T, next: Option<Box<Node<T>>>) -> Self {
        Node { value: val, next: next, }
    }
}

pub struct SinglyLinkedList<T> {
    size: usize,
    front: Option<Box<Node<T>>>,
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList { size: 0, front: None }
    }

    fn push_front(&mut self, item: T) {
        let new_front = Some(Box::new(Node::new(item, mem::replace(&mut self.front, None))));
        self.front = new_front;
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        match self.front.take() {
            Some(mut n) => {
                self.front = n.next.take();
                self.size -= 1;
                Some(n.value)
            },
            _ => None,
        }
    }

    fn peek_front(&self) -> Option<&T> {
        match self.front {
            Some(ref n) => Some(&n.value),
            _ => None,
        }
    }

    fn len(&self) -> usize {
        self.size
    }
}
