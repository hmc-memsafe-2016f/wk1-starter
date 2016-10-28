// Zach Hauser <zachary.hauser@pomona.edu>
// Submission for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;

pub struct SinglyLinkedList<T> {
    head: Option<Node<T>>,
    size: usize
}

struct Node<T> {
    item : T,
    next: Box<Option<Node<T>>>,
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {

    fn new() -> Self {
        SinglyLinkedList { head: None, size: 0 }
    }

    fn push_front(&mut self, item: T) {
        self.size += 1;
        let mut node = Node { item: item, next: Box::new(None) };
        if self.head.is_some() {
            node.next = Box::new(self.head.take());
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
                (*head.next, Some(head.item))
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

    fn remove_first(&mut self, target: &T) -> Option<T> { 
        let prev_maybe : &mut Option<Node<T>> = &mut None;
        let node_maybe = &mut self.head;

        if node_maybe.is_none() {
            return None;
        }

        while let &mut Some(ref mut node) = node_maybe {
            if node.item == *target {
                return match prev_maybe {
                    &mut None => {
                        self.head = None;
                        node_maybe.take().map(|result_node| result_node.item)
                    }
                    &mut Some(ref mut node) => {
                        None
                    }
                }
                //return Some(node.item)
            }
        }
        unimplemented!()
    }

}