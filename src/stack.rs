// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use std::mem;
use Stack;

struct Node<T> {
    pointer: Option<Box<Node<T>>>,
    data: T,
}

pub struct SinglyLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList{head: None, size: 0}
    }

    fn push_front(&mut self, item: T) {
        let head = Box::new(Node {
            pointer: self.head.take(),
            data: item,
        });
        self.head = Some(head);
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|n| {
            assert!(self.size > 0);
            let node = *n;
            self.head = node.pointer;
            self.size -= 1;
            node.data
        })
    }

    fn peek_front(&self) -> Option<&T> {
        match self.head {
            Some(ref node) => Some(&node.data),
            None => None
        }
    }

    fn len(&self) -> usize { self.size }

    fn remove_first(&mut self, item: &T) -> Option<T> {
        let mut list = SinglyLinkedList::new();
        let mut result = None;
        let mut result_found = false;

        loop {
            match self.head.take() {
                Some(ref n) => {
                    if !result_found && n.data == *item {
                        result = self.pop_front();
                        result_found = true;
                    } else {
                        list.push_front(self.pop_front().unwrap());
                    }
                },
                None => break
            }
        }

        list.reverse();
        mem::swap(&mut self.head, &mut list.head);

        result
    }

    fn reverse(&mut self) {
        let mut list = SinglyLinkedList::<T>::new();

        loop {
            match self.head {
                Some(_) => list.push_front(self.pop_front().unwrap()),
                None => break
            }
        }

        mem::swap(&mut self.head, &mut list.head);
    }
}
