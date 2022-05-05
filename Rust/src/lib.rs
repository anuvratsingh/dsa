#![allow(non_snake_case, dead_code)]

mod Algorithms;
mod DataStructures;

#[cfg(test)]
mod test {
    use std::{cell::RefCell, fmt::Debug};

    use rand::seq::SliceRandom;

    use crate::DataStructures::{
        BinarySearchTree::{DeviceRegistry, IoTDevice},
        DoublyLinkedList::BetterTransactionLog,
        DynamicArray::TimestampSaver,
        SinglyLinkedList::TransactionLog,
        SkipList::BestTransactionLog,
    };

    fn new_device_with_id<I: Debug + Copy, A: From<String>>(id: I) -> IoTDevice<I, A> {
        IoTDevice::new(
            id,
            format!("Address is {:?}", id),
            format!("Path is {:?}", id),
        )
    }

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

    // Dynamic Array
    #[test]
    fn da() {
        let mut array = TimestampSaver::new_empty();
        assert_eq!(array.at(1), None);

        array.append("One");
        array.append("Two");

        assert_eq!(array.at(0), Some("One"));
        assert_eq!(array.at(1), Some("Two"));
    }
    // Binary Search Tree
    #[test]
    fn bst() {
        let mut tree = DeviceRegistry::<usize, &str>::new_empty();
        assert_eq!(tree.find(10), None);

        tree.add(IoTDevice::new(1, "One Add", "One Path"));
        tree.add(IoTDevice::new(2, "Two Add", "Two Path"));
        tree.add(IoTDevice::new(3, "Three Add", "Three Path"));

        assert_eq!(tree.find(2), Some(IoTDevice::new(2, "Two Add", "Two Path")));
        assert_eq!(
            tree.find(3),
            Some(IoTDevice::new(3, "Three Add", "Three Path"))
        );
    }

    #[test]
    fn bst_walk_in_order() {
        let len = 10;
        let mut tree = DeviceRegistry::<usize, String>::new_empty();
        let mut items: Vec<IoTDevice<usize, String>> =
            (0..len).map(|id| new_device_with_id(id)).collect();

        let mut rng = rand::thread_rng();
        items.shuffle(&mut rng);

        for item in items.iter() {
            tree.add(item.clone());
        }

        assert_eq!(tree.length, len);

        let v: RefCell<Vec<IoTDevice<usize, String>>> = RefCell::new(Vec::new());
        tree.walk(|n| v.borrow_mut().push(n.clone()));

        let mut items = items;

        items.sort_by(|a, b| b.id.cmp(&a.id));
        assert_eq!(v.into_inner(), items);
    }
}
