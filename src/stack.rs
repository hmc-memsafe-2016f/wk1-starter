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

    fn remove_first(&mut self, item : &T) -> Option<T> {
        unimplemented!(); // commented out impl so my submission would compile
        /*
        use std::mem;
        // make sure the first isn't something we should remove
        let mut current = &self.first;
        loop {
            match current.as_mut() {
                Some(ref mut node) => {
                    // trying to take ownership and give it back if it isn't the
                    // first node that has a payload equal to item
                    let mut next = node.next.take();
                    // this seems to be an issue because ownership next gets moved
                    let remove_next = next.map_or(false, |n| &(n.payload) == item);
                    if remove_next {
                        // can't panic because next.map_or() didn't return default value 
                        node.next = next.unwrap().next;
                        return Some(node.payload);
                    } else {
                        // give ownership back
                        mem::replace(&mut node.next, next);
                        current = &(node.next);
                    }
                },
                None => {return None;},
            }
        }
        */
    }

    fn reverse(&mut self) {
        use std::mem;
        let mut current = self.first.take();
        let mut prev = None;
        loop {
            // following is now current's next (taking ownership)
            // and current now points to prev
            let following = current.as_mut().and_then(
                |ref mut node| mem::replace(&mut node.next, prev));
            if following.is_none() { break; }
            // update for next iteration of loop
            prev = current;
            current = following;
        }
        self.first = current;
    }
}

