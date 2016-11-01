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
    	//create a new node with the data
    	let mut new_first = Node{data: item, next: None};

    	//new first node takes ownership of the old first node
    	mem::swap(&mut new_first.next, &mut self.first);
    	//now new_first has self.first and self.first has None

    	//give the list ownership of the new first node
    	self.first = Some(Box::new(new_first));
    	self.length += 1;
    }

    fn pop_front(&mut self) -> Option<T>
    {
		//replace the first element to take ownership of it
		let old_node = mem::replace(&mut self.first, None);

		match old_node
		{
			//If there was a node, dereference it,
			//replace first with its next
			//we removed a node so length goes down
			//and return its data.
			Some(box_node) => {
								let node = *box_node;
								self.first = node.next;
								self.length -= 1;
								Some(node.data)
							 },
			//If we didn't have a node to pop, just return none
			//and put none back on first.
			None => {
						//not strictly necessary...
						mem::replace(&mut self.first, None);
						None
					},
		}
    }

    fn peek_front(& self) -> Option<&T>
    {
    	self.first.as_ref().map(|node| &node.data)
    }

    fn len(&self) -> usize
    {
    	self.length
    }

	/*
    fn remove_first(&mut self, _: &T) -> Option<T> { None }
    {
    	
    }

    fn reverse(&mut self)
    {
		//needs to take ownership of each node in turn and push it back onto the stack
    }
    */
}
