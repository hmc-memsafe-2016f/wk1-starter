// Michael Sheely <msheely@hmc.edu>
// Developed from the Starter code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;

pub struct SinglyLinkedList<T> {
    first: Option<Box<Node<T>>>,
    size: usize,
}

struct Node<T> {
    payload: T,
    next: Option<Box<Node<T>>>,
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList{first: None, size: 0}
    }

    fn push_front(&mut self, item: T) {
        self.first = Some(Box::new(Node{payload: item, next: self.first.take()}));
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        if self.size > 0 {self.size -= 1;}
        self.first.take().map(|mut node| {self.first = node.next.take(); node.payload})
    }
    
    fn peek_front(&self) -> Option<&T> {
        match self.first {
            Some(ref node) => Some(&node.payload),
            None => None,
        }
    }

    fn len(&self) -> usize { self.size }

    //fn remove_first(&mut self, _: &T) -> Option<T> {}
    /*
    fn reverse(&mut self) {
        let mut node : Node<T>;
        match self.first {
            Some(x) => {node = *x;},
            None => {return;},
        }
        loop {
            match node.next {
                Some(next_node) => next_node.next.take(),
                None => self.first,
            };
            // let mut next = node.next.take();
            // node = //the next one
        }
    }
    */
    // ????
}
