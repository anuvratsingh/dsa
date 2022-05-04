use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

#[derive(Debug, Clone)]
struct Node<T> {
    value: T,
    next: Link<T>,
    prev: Link<T>,
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

// Access & Search are Avg O(n)
// Insertion and Deletion are Avg O(1)
// Positives
//    1.Low overhead allocation per item (but more than the singly
//    linked list).
//    2. Item count is only limited by heap memory.
//    3. Mutation while iterating is possible.
//    4.Implementation is more complex but still fairly simple.
//    5. Inserts, deletes, append, and prepend remain efficient.
//    6. Efficient reversion.

// Negatives
//    1. Indexing is still inefficient.
//    2. Nodes are also allocated on the heap, which requires a lot of
//    jumping around too.
//    3. An additional pointer has to be stored per node.
//    4. Implementation is more complex.

#[derive(Debug, Clone)]
pub struct BetterTransactionLog<T> {
    head: Link<T>,
    tail: Link<T>,
    pub length: u64,
}

impl<T> Node<T> {
    fn new(value: T) -> Rc<RefCell<Node<T>>> {
        Rc::new(RefCell::new(Node {
            value: value,
            next: None,
            prev: None,
        }))
    }
}

impl<T> BetterTransactionLog<T> {
    pub fn new_empty() -> Self {
        BetterTransactionLog {
            head: None,
            tail: None,
            length: 0,
        }
    }
    pub fn append_fwd(&mut self, value: T) {
        let new_head = Node::new(value);
        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head)
            }
            None => self.tail = Some(new_head.clone()),
        }
        self.length += 1;
        self.head = Some(new_head);
    }
    pub fn append_bwd(&mut self, value: T) {
        let new_head = Node::new(value);
        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_head.clone());
                new_head.borrow_mut().prev = Some(old_tail)
            }
            None => self.head = Some(new_head.clone()),
        }
        self.length += 1;
        self.tail = Some(new_head);
    }

    pub fn pop_fwd(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            if let Some(next) = old_head.borrow_mut().next.take() {
                next.borrow_mut().prev.take();
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(old_head)
                .ok()
                .expect("Panic!")
                .into_inner()
                .value
        })
    }

    pub fn pop_bwd(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            if let Some(prev) = old_tail.borrow_mut().prev.take() {
                prev.borrow_mut().next.take();
                self.tail = Some(prev);
            } else {
                self.head.take();
            }
            self.length -= 1;
            Rc::try_unwrap(old_tail)
                .ok()
                .expect("Panic!")
                .into_inner()
                .value
        })
    }

    pub fn peek_fwd(&self) -> Option<Ref<T>> {
        self.head
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }

    pub fn peek_bwd(&self) -> Option<Ref<T>> {
        self.tail
            .as_ref()
            .map(|node| Ref::map(node.borrow(), |node| &node.value))
    }
    pub fn back_iter(self) -> ListIterator<T> {
        ListIterator::new(self.tail)
    }

    pub fn iter(self) -> ListIterator<T> {
        ListIterator::new(self.head.clone())
    }
}

pub struct ListIterator<T> {
    current: Link<T>,
}

impl<T> ListIterator<T> {
    fn new(start_at: Link<T>) -> ListIterator<T> {
        ListIterator { current: start_at }
    }
}

impl<T> Iterator for ListIterator<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None,
        };
        result
    }
}

impl<T> DoubleEndedIterator for ListIterator<T>
where
    T: Clone,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            }
            None => None,
        };
        result
    }
}
