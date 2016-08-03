// Alex Ozdemir <aozdemir@hmc.edu>
// Sample solution for HMC's MemSafe, Fall 2016, Week 1
//
// The implementation of SinglyLinkedList
use Stack;

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: Link<T>,
}

pub struct SinglyLinkedList<T: Eq> {
    head: Link<T>,
    len: usize,
}

fn remove_first<T: Eq>(link: Link<T>, item: &T) -> (Link<T>, Option<T>) {
    link.map_or((None, None), |mut node| {
        if *item == node.data {
            // See the comment in `pop_front` for why we need to `take` here
            (node.next.take(), Some(node.data))
        } else {
            // While less obvious, we need to use `take` here for the same
            // reason as in `pop_front` and above
            let (new_next, removed) = remove_first(node.next.take(), item);
            node.next = new_next;
            (Some(node), removed)
        }
    })
}

impl<T: Eq> Stack<T> for SinglyLinkedList<T> {
    fn new() -> Self {
        SinglyLinkedList {
            head: None,
            len: 0,
        }
    }

    fn push_front(&mut self, item: T) {
        self.head = Some(Box::new(Node {
            data: item,
            next: self.head.take(),
        }));
        self.len += 1;
    }

    fn peek_front(&self) -> Option<&T> {
        // Good question: Why `as_ref`?
        self.head.as_ref().map(|ref node| &node.data)
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|mut node| {
            self.len -= 1;
            // We really just want to move the two members of the node to
            // different locations, but (as of May 2016) borrowck sees
            // movement of any member as movement of whole, so this is not
            // possible. `take` allows us to get the value of node.next by
            // replacing it with None, so it doesn't count as a move.
            //
            // Fragment analysis in borrowck will allow separate movement
            // of members. Soon plz.
            self.head = node.next.take();
            node.data
        })
    }

    fn len(&self) -> usize {
        self.len
    }

    fn remove_first(&mut self, item: &T) -> Option<T> {
        let head = self.head.take();
        let (new_head, removed) = remove_first(head, item);
        self.head = new_head;
        if removed.is_some() {
            self.len -= 1;
        }
        removed
    }

    fn reverse(&mut self) {
        let mut tmp = Self::new();
        while let Some(item) = self.pop_front() {
            tmp.push_front(item);
        }
        *self = tmp;
    }
}

impl<T: Eq> Drop for SinglyLinkedList<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop_front() { };
    }
}
