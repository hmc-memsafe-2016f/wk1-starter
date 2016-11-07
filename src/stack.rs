// Jackson Warley
//
// The implementation of SinglyLinkedList

use Stack;
use std::mem;

type NodePointer<T> = Option<Box<Node<T>>>;

struct Node<T> {
	value: T,
	next: NodePointer<T>,
}

impl<T: Eq> Node<T> {
	fn new(value: T, next: NodePointer<T>) -> Self {
		Node { value: value, next: next }
	}
}

pub struct SinglyLinkedList<T> {
    size: usize,
    head: NodePointer<T>,
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
	fn new() -> Self
	{
		SinglyLinkedList { size: 0, head: None }
	}

    fn push_front(&mut self, item: T)
    {
        let mut pushee = Node::new(item, None);
        mem::swap(&mut self.head, &mut pushee.next);
        self.head = Some(Box::new(pushee));

        self.size += 1;
	}

    fn pop_front(&mut self) -> Option<T>
    {
        let front = self.head.take();
        match front {
            None => None,
            Some(mut front_box) => {
                self.head = front_box.next.take();
                self.size -= 1;
                return Some(front_box.value)
            }
        }
	}

    fn peek_front(&self) -> Option<&T>
    {
        match self.head {
            None => None,
            Some(ref front) => Some(&front.value),
        }
	}

    fn len(&self) -> usize
    {
        self.size
	}

	fn remove_first(&mut self, _: &T) -> Option<T>
	{
		unimplemented!();
	}

    fn reverse(&mut self)
    {
    	unimplemented!();
    }
}
