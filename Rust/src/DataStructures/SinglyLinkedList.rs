use std::{cell::RefCell, rc::Rc};

type Link<T> = Rc<RefCell<Node<T>>>;
#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Link<T>>,
}
// Requirements
//      1. Append a command to an existing list.
//      2. Replay every command from the beginning to the end—in
//      that order.

// Access & Search are Avg O(n) 
// Insertion and Deletion are Avg O(1)

// Positives 
//      1. Low overhead allocation per item.
//      2. Item count is only limited by heap memory.
//      3. Mutation while iterating is possible.
//      4. A direction is strictly enforced—there is no going back.
//      5. Implementation is fairly simple (even in Rust).
//      6. Efficient append, prepend, delete, and insert
//      operations—compared to an array (no shifting required).

// Negatives 
//      1. Indexing is inefficient, since every node has to be looked at.
//      Iteration in general involves a lot of jumping around on the
//      heap, which takes more time and makes the operation hard to cache.
//      2. Reversing a list is very inefficient.
#[derive(Clone)]
pub struct TransactionLog<T> {
    head: Option<Link<T>>,
    tail: Option<Link<T>>,
    pub length: u64,
}

impl<T> Node<T> {
    fn new(value: T) -> Link<T> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
        }))
    }
}

impl<T> TransactionLog<T> {
    pub fn new_empty() -> Self {
        TransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn append(&mut self, value: T) {
        let new = Node::new(value);
        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        }
        self.length += 1;
        self.tail = Some(new);
    }
    // Pops from front
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Panic!")
                .into_inner()
                .value
        })
    }
}
