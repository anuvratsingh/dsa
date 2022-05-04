#![allow(non_snake_case, dead_code)]

mod Algorithms;
mod DataStructures;

#[cfg(test)]
mod test {
    use crate::DataStructures::{
        DoublyLinkedList::BetterTransactionLog, SinglyLinkedList::TransactionLog,
        SkipList::BestTransactionLog,
    };

    // Singly Linked List SLL
    #[test]
    fn sll() {
        let mut list = TransactionLog::<&str>::new_empty();

        assert_eq!(list.pop(), None);
        list.append("One");
        list.append("Two");
        list.append("Three");

        assert_eq!(list.pop(), Some("One"));
        assert_eq!(list.pop(), Some("Two"));

        list.append("Four");
        list.append("Five");
        assert_eq!(list.pop(), Some("Three"));
        assert_eq!(list.pop(), Some("Four"));
        assert_eq!(list.pop(), Some("Five"));

        assert_eq!(list.pop(), None);
    }

    // Doubly Linked List DLL
    #[test]
    fn dll() {
        let mut list = BetterTransactionLog::<&str>::new_empty();
        assert_eq!(list.pop_bwd(), None);
        assert_eq!(list.pop_fwd(), None);
        assert!(list.peek_fwd().is_none());
        assert!(list.peek_bwd().is_none());

        list.append_fwd("One");
        list.append_bwd("Two");

        assert_eq!(list.pop_bwd(), Some("Two"));
        assert_eq!(list.pop_fwd(), Some("One"));

        list.append_bwd("Three");
        list.append_fwd("Two");
        list.append_bwd("Four");
        list.append_fwd("One");

        assert_eq!(&*list.peek_fwd().unwrap(), &"One");
        assert_eq!(&*list.peek_bwd().unwrap(), &"Four");

        assert_eq!(list.pop_bwd(), Some("Four"));
        assert_eq!(list.pop_fwd(), Some("One"));
        assert_eq!(list.pop_bwd(), Some("Three"));
        assert_eq!(list.pop_fwd(), Some("Two"));
        assert_eq!(list.pop_bwd(), None);
        assert_eq!(list.pop_fwd(), None);
    }

    // Add test for DLL Iter

    // Skip List
    #[test]
    fn sl() {
        let mut list = BestTransactionLog::<usize, &str>::new_empty(5);
        assert_eq!(list.find(5), None);

        list.append(1, "One");
        list.append(4, "Four");
        list.append(6, "Six");
        list.append(7, "Seven");
        list.append(11, "Eleven");
        list.append(13, "Thirteen");

        assert_eq!(list.find(1), Some("One"));
        assert_eq!(list.find(11), Some("Eleven"));
        assert_eq!(list.find(7), Some("Seven"));
        assert_eq!(list.find(13), Some("Thirteen"));
        assert_eq!(list.find(18), None);
    }
}
