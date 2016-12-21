extern crate wk1;

pub use wk1::{Stack,SinglyLinkedList};

#[derive(PartialEq,Eq,Debug)]
struct NoCopy(isize);

// This macro is an assertion with nicely formatted failure output
macro_rules! assert_expected_eq_actual {
    ($a:expr, $b:expr) => ({
        let (a, b) = (&$a, &$b);
        assert!(*a == *b,
                "\nExpected `{:?}` is not equal to Actual `{:?}`\nAssertion: `assert_expected_eq_actual!({}, {})`",
                *a,
                *b,
                stringify!($a),
                stringify!($b));
    })
}

// Tests you're required to pass
mod required {

    use super::{NoCopy,SinglyLinkedList,Stack};

    // This attribute identifies this function as a unit test
    #[test]
    fn new() {
        let s = SinglyLinkedList::<i64>::new();
        assert_expected_eq_actual!(0, s.len());
    }

    #[test]
    fn push() {
        let mut s = SinglyLinkedList::new();
        s.push_front(5);
        assert_expected_eq_actual!(1, s.len());
    }

    #[test]
    fn push_pop() {
        let mut s = SinglyLinkedList::new();
        s.push_front(5);
        assert_expected_eq_actual!(Some(5), s.pop_front());
        assert_expected_eq_actual!(0, s.len());
    }

    #[test]
    fn push_pop_pop() {
        let mut s = SinglyLinkedList::new();
        s.push_front(5);
        assert_expected_eq_actual!(Some(5), s.pop_front());
        assert_expected_eq_actual!(0, s.len());
        assert_expected_eq_actual!(None, s.pop_front());
        assert_expected_eq_actual!(0, s.len());
    }

    #[test]
    fn push_pop_push_pop() {
        let mut s = SinglyLinkedList::new();
        s.push_front(5);
        assert_expected_eq_actual!(Some(5), s.pop_front());
        assert_expected_eq_actual!(0, s.len());
        s.push_front(6);
        assert_expected_eq_actual!(Some(6), s.pop_front());
        assert_expected_eq_actual!(0, s.len());
    }

    #[test]
    fn push_push_pop_pop() {
        let mut s = SinglyLinkedList::new();
        s.push_front(5);
        s.push_front(6);
        assert_expected_eq_actual!(2, s.len());
        assert_expected_eq_actual!(Some(6), s.pop_front());
        assert_expected_eq_actual!(Some(5), s.pop_front());
        assert_expected_eq_actual!(0, s.len());
    }

    #[test]
    fn push_pop_many() {
        let mut s = SinglyLinkedList::new();
        for i in 0 .. 1000 {
            s.push_front(i);
            assert_expected_eq_actual!(1, s.len());
            assert_expected_eq_actual!(Some(i), s.pop_front());
            assert_expected_eq_actual!(0, s.len());
        }
    }

    #[test]
    fn pushs_pops() {
        let mut s = SinglyLinkedList::new();
        for i in 0 .. 1000 {
            s.push_front(i);
            assert_expected_eq_actual!(i + 1, s.len());
        }
        for i in (0 .. 1000).rev() {
            assert_expected_eq_actual!(Some(i), s.pop_front());
            assert_expected_eq_actual!(i, s.len());
        }
    }

    #[test]
    fn push_peek() {
        let mut s = SinglyLinkedList::new();
        s.push_front(NoCopy(5));
        assert_expected_eq_actual!(1, s.len());
        let expected = NoCopy(5);
        assert_expected_eq_actual!(Some(&expected), s.peek_front());
        assert_expected_eq_actual!(1, s.len());
    }

    #[test]
    fn push_peek_peek_pop() {
        let mut s = SinglyLinkedList::new();
        s.push_front(NoCopy(5));
        assert_expected_eq_actual!(1, s.len());
        let expected = NoCopy(5);
        assert_expected_eq_actual!(Some(&expected), s.peek_front());
        assert_expected_eq_actual!(1, s.len());
        assert_expected_eq_actual!(Some(&expected), s.peek_front());
        assert_expected_eq_actual!(1, s.len());
        assert_expected_eq_actual!(Some(NoCopy(5)), s.pop_front());
        assert_expected_eq_actual!(0, s.len());
    }
}

// Tests for assignment bonuses!
mod bonus {

    mod remove_first {
        use super::super::{NoCopy,Stack,SinglyLinkedList};

        #[test]
        #[ignore]
        fn push1_remove1() {
            let mut s = SinglyLinkedList::new();
            s.push_front(NoCopy(1));
            assert_expected_eq_actual!(Some(NoCopy(1)), s.remove_first(&NoCopy(1)));
            assert_expected_eq_actual!(0, s.len());
        }

        #[test]
        #[ignore]
        fn push1_push1_remove1_remove1_remove1() {
            let mut s = SinglyLinkedList::new();
            s.push_front(NoCopy(1));
            s.push_front(NoCopy(1));
            assert_expected_eq_actual!(Some(NoCopy(1)), s.remove_first(&NoCopy(1)));
            assert_expected_eq_actual!(1, s.len());
            assert_expected_eq_actual!(Some(NoCopy(1)), s.remove_first(&NoCopy(1)));
            assert_expected_eq_actual!(0, s.len());
            assert_expected_eq_actual!(None, s.remove_first(&NoCopy(1)));
            assert_expected_eq_actual!(0, s.len());
        }

        #[test]
        #[ignore]
        fn push1_push2_remove1() {
            let mut s = SinglyLinkedList::new();
            s.push_front(NoCopy(1));
            s.push_front(NoCopy(2));
            assert_expected_eq_actual!(Some(NoCopy(1)), s.remove_first(&NoCopy(1)));
            assert_expected_eq_actual!(1, s.len());
            assert_expected_eq_actual!(Some(NoCopy(2)), s.pop_front());
            assert_expected_eq_actual!(0, s.len());
        }

        #[test]
        #[ignore]
        fn push1_push2_remove3() {
            let mut s = SinglyLinkedList::new();
            s.push_front(NoCopy(1));
            s.push_front(NoCopy(2));
            assert_expected_eq_actual!(None, s.remove_first(&NoCopy(3)));
            assert_expected_eq_actual!(2, s.len());
        }
    }

    mod reverse {
        use super::super::{NoCopy,Stack,SinglyLinkedList};

        #[test]
        #[ignore]
        fn reverse_1() {
            let mut s = SinglyLinkedList::new();
            s.push_front(NoCopy(1));
            s.reverse();
            assert_expected_eq_actual!(Some(NoCopy(1)), s.pop_front());
        }

        #[test]
        #[ignore]
        fn reverse_many() {
            let mut s = SinglyLinkedList::new();
            for i in 0..100 {
                s.push_front(NoCopy(i));
            }
            s.reverse();
            for i in 0..100 {
                assert_expected_eq_actual!(Some(NoCopy(i)), s.pop_front());
            }
            assert_expected_eq_actual!(None, s.pop_front());
        }
    }

    mod mystery {
        use super::super::{Stack,SinglyLinkedList};

        #[test]
        #[ignore]
        fn wat() {
            let mut s = SinglyLinkedList::new();
            let magic = 1 * 1024 * 1024;
            for i in 0..magic {
                s.push_front(i);
            }
        }
    }
}
