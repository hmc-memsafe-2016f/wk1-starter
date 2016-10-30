// Ross Mawhorter <rmawhorter@g.hmc.edu>
// Starter code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;
use std::mem;

//TODO: need to BOX?
pub struct SinglyLinkedList<T> {
    first: Option<Box<Node<T>>>,
    length: usize
}

pub struct Node<T> {
	data: T,
	next: Option<Box<Node<T>>>
}

//implement Eq for Node?

//implement Stack<T> functions for SinglyLinkedList
impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> Self
    {
    	SinglyLinkedList{first: None, length: 0}
    }

    fn push_front(&mut self, item: T)
    {
    	let mut new_first = Node{data: item, next: None};

    	//TWO things need to happen here
    	//1. the new node gets first as its next
    	//2. the first gets the boxed new node

    	//need to take ownership of the first node in order to give it to the second node
    	//the list needs to take ownership of the new node.

    	mem::swap(&mut new_first.next, &mut self.first);
    	//now new_first has self.first and self.first has None

    	self.first = Some(Box::new(new_first));
    	self.length += 1;
    }

    fn pop_front(&mut self) -> Option<T>
    {
		//create an empty option node
		let empty_node = None;

		//replace the first element and take ownership of it
		let old_node = mem::replace(&mut self.first, empty_node);

		match old_node
		{
			//If there was a node, dereference it,
			//replace first with its next
			//we removed a node so length goes down
			//and return its data.
			Some(box_node) => {
								let node = *box_node;
								mem::replace(&mut self.first, node.next);
								self.length -= 1;
								Some(node.data)
							 },
			//If we didn't have a node to pop, just return none
			//and put none back on first.
			None => {
						mem::replace(&mut self.first, None);
						None
					},
		}
    }

    fn peek_front(& self) -> Option<&T>
    {
    	//TODO: no match
    	self.first.as_ref().map(|node| &node.data)
    }

    fn len(&self) -> usize
    {
    	self.length
    }

	/*
    fn remove_first(&mut self, _: &T) -> Option<T> { None }
    {
    	None
    }

    fn reverse(&mut self)
    {

    }
    */
}
