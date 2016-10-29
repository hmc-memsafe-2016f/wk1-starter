// Alex Ozdemir <aozdemir@hmc.edu> // <- Your name should replace this line!
// Starter code for HMC's MemorySafe, week 1
//
// The implementation of SinglyLinkedList

use Stack;

pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>
}

pub struct SinglyLinkedList<T> {
    first : Option<Box<Node<T>>>
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> Self 
    {
        SinglyLinkedList{first: None}
    }
    fn push_front(&mut self, item: T)
    {
        let nextNode = match self.first.take() {
            None => None,
            Some(d) => Some(d)
        };
        self.first = Some(Box::<Node<T>>::new(Node{data: item, next: nextNode}));
    }
    fn pop_front(&mut self) -> Option<T>
    {
        let mut retOption = self.first.take();
        let mut retVal : Option<T> = match retOption {
            None => None,
            Some(d) => Some(d.data)
        };
        self.first = match retOption {
            None => None,
            Some(d) => d.next.take()
        };
        retVal
    }
    fn peek_front(&self) -> Option<&T> {unimplemented!()}
    fn len(&self) -> usize {unimplemented!()}
}