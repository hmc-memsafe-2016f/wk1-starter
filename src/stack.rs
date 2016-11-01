// Adam Dunlap <adunlap@hmc.edU>
// Starter code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;
use std::mem;

pub struct SinglyLinkedList<T> {
    sz: usize,
    fst: Node<T>,
}

enum Node<T> {
    Nil,
    Cons(T, Box<Node<T>>),
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList{sz: 0, fst: Node::Nil}
    }
    fn push_front(&mut self, item: T) {
        // Take ownership of the rest of the list by replacing it with Nil
        let oldfst = mem::replace(&mut self.fst, Node::Nil);

        self.fst = Node::Cons(item, Box::new(oldfst));
        self.sz += 1;
    }
    fn pop_front(&mut self) -> Option<T> {
        // Take ownership of the whole list
        let oldfst = mem::replace(&mut self.fst, Node::Nil);
        match oldfst {
            Node::Nil => None,
            Node::Cons(ret, newfst) => {
                self.sz -= 1;
                // Put the rest of the list back into self
                mem::replace(&mut self.fst, *newfst);
                Some(ret)
            }
        }
    }
    fn peek_front(&self) -> Option<&T> {
        match self.fst {
            Node::Nil => None,
            Node::Cons(ref ret, _) => Some(ret)
        }
    }
    fn len(&self) -> usize {
        self.sz
    }

    fn remove_first(&mut self, to_remove: &T) -> Option<T> {
        // Remove an element by pushing and popping all but the first instance
        // of the element we want to remove which reverses it. Then, reverse it
        // again.
        let mut rev_list = SinglyLinkedList::new();
        let mut ret = None;
        while let Some(v) = self.pop_front() {
            if v == *to_remove {
                ret = Some(v);
                break;
            }
            rev_list.push_front(v);
        }
        while let Some(v) = self.pop_front() {
            rev_list.push_front(v);
        }
        while let Some(v) = rev_list.pop_front() {
            self.push_front(v);
        }
        ret
    }
    fn reverse(&mut self) {
        let mut rev_list = SinglyLinkedList::new();
        while let Some(v) = self.pop_front() {
            rev_list.push_front(v);
        }
        *self = rev_list;
    }
}

// The default destructor hits a stack overflow for large lists because of the
// chained ownership. To get around this, write a destructor that destructs all
// the nodes one at a time in a loop, so the stack doesn't grow at all.
impl<T> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        // Take ownership of the first node
        let mut cur: Node<T> = mem::replace(&mut self.fst, Node::Nil);
        while let Node::Cons(_, next) = cur {
            // Take owenership of the next node
            cur = *next;
            // As cur goes out of scope, drop the node. We've already
            // transferred ownership of its next, so only one will be dropped at
            // a time, so we don't get a stack overflow
        }
    }
}
