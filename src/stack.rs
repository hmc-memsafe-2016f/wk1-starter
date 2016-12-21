// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;
use std::mem;


pub struct SinglyLinkedList<T> {
    head: Option<Box<(Node<T>)>>,
    len: usize,
}

struct Node<T>{
    value: T,
    next: Option<Box<(Node<T>)>>
}

impl <T> Node<T>{
    fn new(value : T, next: Option<Box<(Node<T>)>>) -> Node<T>{
        Node{
            value : value,
            next  : next,
        }
    }
}


impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> SinglyLinkedList<T>{
        SinglyLinkedList::<T>{
            head: None,
            len: 0,
        }
    }

    fn push_front(&mut self, item:T) -> (){
        let mut new_head = Node::new(item, None);
        mem::swap(&mut new_head.next, &mut self.head);
        self.head = Some(Box::new(new_head));

        self.len += 1;
    } 

    fn pop_front(&mut self) -> Option<T>{
        match Option::take(&mut self.head) {
            None => None,
            Some(node) => {
                            let node = *node;
                            self.head = node.next;
                            self.len -= 1;
                            Some(node.value)
            }
        }
    }
    
    fn peek_front(&self) -> Option<&T>{
        match self.head {
            None => None,
            Some(ref node) => Some(&(node.value)),
        }
    }   

    fn len(&self) -> usize{
        self.len
    }

    fn remove_first(&mut self, _: &T) -> Option<T>{
        unimplemented!()
    }

    fn reverse(&mut self){
        unimplemented!()
    }
}
