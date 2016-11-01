// Julien Chien <jchien17@cmc.edu>
// Starter code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;

struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList{ head: None, size: 0 }
    }

    fn push_front(&mut self, item: T) {
        let newnode = Box::new(Node{ value: item, next: self.head.take() });

        self.head = Some(newnode);
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            let oldhead =  *node;
            self.head = oldhead.next;
            self.size -= 1;
            oldhead.value
        })
    }

    fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {&node.value})
    }

    fn len(&self) -> usize {
        self.size
    }

}
