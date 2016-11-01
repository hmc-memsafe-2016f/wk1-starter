// Julien Chien <jchien17@cmc.edu>
// Starter code for HMC's MemorySafe, week 1
//
// The trait Stack, and a re-export of SinglyLinkedList, which implements it


mod stack;

pub use stack::SinglyLinkedList;

pub trait Stack<T> {
    fn new() -> Self;
    fn push_front(&mut self, item: T);
    fn pop_front(&mut self) -> Option<T>;
    fn peek_front(&self) -> Option<&T>;
    fn len(&self) -> usize;
    fn remove_first(&mut self, _: &T) -> Option<T> { None }
    fn reverse(&mut self) { }
}
