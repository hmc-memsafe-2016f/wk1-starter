// Luis Viornery <lviornery@hmc.edu>
// Significant help from Ross Mawhorter
// Code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;
use std::mem;


pub struct Node<T> {
//We have a node of the struct with a data and a next_node member
//all Nodes in a lost should be Boxed
    data: T,
    //next_node may be None, or it may be a boxed node
    next_node: Option<Box<Node<T>>>,
}

pub struct SinglyLinkedList<T> {
//The singlylinkedlist has no data member, and it has a length
    top_node: Option<Box<Node<T>>>,
    length: usize,
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    //make a new list with no top node and 0 length
    fn new() -> Self {
        SinglyLinkedList{
            top_node: None,
            length: 0,
        }
    }
    fn push_front(&mut self, item: T) {
    //add 1 to length, then create a new boxed node
        self.length += 1;
        let mut new_box = Box::new(Node {
                data: item,
                next_node: None,
            });
        //then make self.topnode point to None
        //and the new box's next_node point to the previous top_node
        mem::swap(&mut self.top_node, &mut new_box.next_node);
        //then make the top node point to the new box
        //(moving gives the list ownership of the box)
        self.top_node = Some(new_box);
    }
    fn pop_front(&mut self) -> Option<T> {
    //replace the top element of the list with the second element if there is one
        //old_top is the Option<Box<Node<T>>> that is the old top_node
        let old_top = mem::replace(&mut self.top_node, None);
        match old_top {
            //If there was no old top node, return nothing
            None => None,
            //If there was...
            Some(boxed_node) => {
                //dereference the box
                let d_b_n = *boxed_node; //defererenced_boxed_node
                //decrement length
                self.length -= 1;
                //move the old next_node to top_node
                self.top_node = d_b_n.next_node;
                //return the data, and ownership of the data
                Some(d_b_n.data)
            }
        }
    }
    fn peek_front(&self) -> Option<&T> {
        //if the top node is not none
        match self.top_node {
            None => None,
            //x is a reference to the boxed node
            Some(ref x) => {
                //return a some-wrapped reference to that data
                //I don't understand why this stays alive
                Some(&x.data)
            },
        }
    }
    fn len(&self) -> usize {
        //ez
        self.length
    }
    /*
    fn remove_first(&mut self, item: &T) -> Option<T> {
        let mut this_node = &mut self.top_node;
        //current_node is the <Option<Box<Node<T>>>> containing the top_node
        //we now own it
        let current_node = mem::replace(this_node, None);
        //to_return gets None by default
        let mut to_return = None;
        loop {
            //assign to_return by matching current_node
            to_return = match current_node {
                //if current_node is none, return none
                None => None,
                //if current_node contains a box, open it up
                Some(boxed_node) => {
                    //dereference the box's contents
                    let d_b_n = *boxed_node; //defererenced_boxed_node
                    //if it matches what we're searching for and
                    //we haven't already found the first item
                    if &d_b_n.data == item && to_return.is_none() {
                        //decrement length
                        self.length -= 1;
                        //replace the previous node with this box's next node and make it own it
                        *this_node = d_b_n.next_node;
                        //THIS ALMOST WORKS I SWEAR, BUT TRAVERSAL IS HARD
                        Some(d_b_n.data)
                    }
                    else {
                        //replace the previous node with this node and make it own it
                        *this_node = Some(Box::new(d_b_n));
                        //AGAIN, TRAVERSAL IS HARD
                        None
                    }
                }
            };
            let current_node = mem::replace(this_node, None);
        }
        to_return
    }
    */
}
