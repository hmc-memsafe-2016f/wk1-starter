// Dan Obermiller <dobermiller16@cmc.edu> // <- Your name should replace this line!
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
        let old = mem::replace(&mut self.head, None);
        self.head = Some(Box::new(Node { pointer: old, data: item }));
        // let mut new_head = Node { pointer: None, data: item };
        // mem::swap(&mut new_head.pointer, &mut self.head);
        // self.head = Some(Box::new(new_head));
        self.size += 1;
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut n| {
            assert!(self.size > 0);
            self.head = n.pointer.take();
            self.size -= 1;
            n.data
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
        // I wanted this to work, but it didn't :(
        // let ref mut current = self.head;
        // let mut previous: Option<Box<Node<T>>> = None;
        //
        // loop {
        //     match current.take() {
        //         Some(n) => {
        //             let unwrapped = *n;
        //             if unwrapped.data == *item {
        //                 let ref mut prev_unwrapped = *previous.unwrap();
        //                 mem::replace(&mut prev_unwrapped.pointer, unwrapped.pointer);
        //                 return Some(unwrapped.data);
        //             }
        //
        //             mem::replace(&mut previous, mem::replace(current, unwrapped.pointer));
        //         },
        //         None => return None
        //     }
        // }

        // Now I have to do it the ugly way - reverse the list, and remove the first
        // occurence, then reverse again.

        let mut tmp = SinglyLinkedList::new();
        let mut result : Option<T> = None;

        while let Some(node) = self.pop_front() {
            if node == *item {
                result = Some(node);
                break;
            }

            tmp.push_front(node);
        }

        while let Some(node) = self.pop_front() {
            tmp.push_front(node);
        }

        tmp.reverse();
        mem::swap(self, &mut tmp);

        result
    }

    fn reverse(&mut self) {
        let mut reversed = SinglyLinkedList::<T>::new();

        while let Some(node) = self.pop_front() {
            reversed.push_front(node);
        }

        mem::swap(self, &mut reversed);
    }
}

// Yay for stack overflow
// The default destructor was a recursive function, which overflows for large lists
impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        let mut current = mem::replace(&mut self.head, None);
        while let Some(node) = current {
            current = node.pointer;
        }
    }
}
