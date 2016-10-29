// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;

pub struct Node<T> {
	data: T,
    next: Option<Box<Node<T>>>
}

pub struct SinglyLinkedList<T> {
    first : Option<Box<Node<T>>>
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> Self 
    {
    	SinglyLinkedList{first: None}
    }
    fn push_front(&mut self, item: T) {unimplemented!()}
    fn pop_front(&mut self) -> Option<T> {unimplemented!()}
    fn peek_front(&self) -> Option<&T> {unimplemented!()}
    fn len(&self) -> usize {unimplemented!()}
}