// Robert "Skipper" Gonzaelz <sgonzalez@hmc.edu>
// Starter code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;
use std::mem;

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    length: usize
}



struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}



impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList{ head: None, length: 0}
    }
    
    
    fn push_front(&mut self, item: T) {
        let new_node =  Node{data: item, next: self.head.take()};
        self.head = Some(Box::new(new_node));
        self.length += 1;
    }
    
    
    fn pop_front(&mut self) -> Option<T> {
        if self.length > 0 {
        
            let old_head = self.head.take();
            let mut unwrapped_old_head = old_head.unwrap();
            
            self.head = mem::replace(&mut unwrapped_old_head.next, None);
            
            let popped = unwrapped_old_head.data;
            
            self.length -= 1;
            Some(popped)
        } else {
            None
        }
    }
    
    
    fn peek_front(&self) -> Option<&T> {
        if self.length > 0 {
            let head_ref = self.head.as_ref();
            Some(&head_ref.unwrap().data)
        } else {
            None
        }
    }
    
    
    fn len(&self) -> usize {
        self.length
    }
}