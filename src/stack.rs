// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;
use std::mem;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

pub struct SinglyLinkedList<T> {
    first : Option<Box<Node<T>>>,
    size : usize
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> Self 
    {
        SinglyLinkedList{first: None, size: 0}
    }
    fn push_front(&mut self, item: T)
    {
        self.size += 1;
        let next_node = match self.first.take() {
            None => None,
            Some(d) => Some(d)
        };
        self.first = Some(Box::<Node<T>>::new(Node{data: item, next: next_node}));
    }
    fn pop_front(&mut self) -> Option<T>
    {
        match self.first.take() {
            None => None,
            Some(mut d) => {self.first = mem::replace(&mut d.next, None);
                            self.size -= 1;
                            Some(d.data)}
        }
    }
    fn peek_front(&self) -> Option<&T>
    {
        match self.first {
            None => None,
            Some(ref d) => Some(& d.data) // why do I need the & here since I already said ref?
        }
    }
    fn len(&self) -> usize
    {
        self.size
    }
}