use std::{
    cell::RefCell,
    fmt::{self, Display},
    rc::Rc,
};

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;
#[derive(Clone)]
struct Node<K, V> {
    next: Vec<Link<K, V>>,
    pub key: K,
    pub value: V,
}
#[derive(Clone)]
pub struct BestTransactionLog<K, V> {
    head: Link<K, V>,
    tails: Vec<Link<K, V>>,
    max_level: usize,
    pub length: u64,
}

impl<K, V> Node<K, V> {
    fn new(next: Vec<Link<K, V>>, key: K, value: V) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node { next, key, value }))
    }
}

impl<K, V> BestTransactionLog<K, V>
where
    K: Display + PartialOrd + Copy,
    V: Clone,
{
    pub fn new_empty(max_level: usize) -> Self {
        BestTransactionLog {
            head: None,
            tails: vec![None; max_level + 1],
            max_level,
            length: 0,
        }
    }
    pub fn append(&mut self, key: K, value: V) {
        let level = 1 + if self.head.is_none() {
            self.max_level
        } else {
            self.get_level()
        };

        let new = Node::new(vec![None; level], key, value);

        for i in 0..level {
            if let Some(old) = self.tails[i].take() {
                let next = &mut old.borrow_mut().next;
                next[i] = Some(new.clone())
            }
            self.tails[i] = Some(new.clone());
        }
        if self.head.is_none() {
            self.head = Some(new.clone());
        }
        self.length += 1;
    }

    pub fn find(&self, key: K) -> Option<V> {
        match self.head {
            Some(ref head) => {
                let mut start_level = self.max_level;
                let node = head.clone();
                let mut result = None;

                loop {
                    if node.borrow().next[start_level].is_none() {
                        break;
                    }
                    start_level -= 1;
                }
                let mut n = node;
                for level in (0..=start_level).rev() {
                    loop {
                        let next = n.clone();
                        match next.borrow().next[level] {
                            Some(ref next) if next.borrow().key <= key => n = next.clone(),
                            _ => break,
                        };
                    }
                    if n.borrow().key == key {
                        let tmp = n.borrow();
                        result = Some(tmp.value.clone());
                        break;
                    }
                }
                result
            }
            None => None,
        }
    }

    fn get_level(&self) -> usize {
        let mut n = 0;
        while rand::random::<bool>() && n < self.max_level {
            n += 1;
        }
        n
    }

    fn iter_level(&self, level: usize) -> ListIterator<K, V> {
        ListIterator::new(self.head.clone(), level)
    }
}
impl<K, V> IntoIterator for BestTransactionLog<K, V>
where
    K: Display + PartialOrd + Copy,
    V: Clone,
{
    type Item = (K, V);

    type IntoIter = ListIterator<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(self.head, 0)
    }
}

pub struct ListIterator<K, V> {
    current: Link<K, V>,
    level: usize,
}

impl<K, V> ListIterator<K, V> {
    fn new(start_at: Link<K, V>, level: usize) -> ListIterator<K, V> {
        ListIterator {
            current: start_at,
            level,
        }
    }
}

impl<K, V> Iterator for ListIterator<K, V>
where
    K: Display + PartialOrd + Copy,
    V: Clone,
{
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        let current = &self.current;
        let mut result = None;

        self.current = match current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some((current.key, current.value.clone()));
                current.next[self.level].clone()
            }
            _ => None,
        };
        result
    }
}

impl<K, V> fmt::Debug for BestTransactionLog<K, V>
where
    K: Display + PartialOrd + Copy,
    V: Clone,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            Some(ref _head) => {
                for level in (0..=self.max_level).rev() {
                    let _ = write!(f, "{}: ", level);
                    for n in self.iter_level(level) {
                        let _ = write!(f, "[{}]", n.0);
                    }
                    let _ = write!(f, "");
                }
                Ok(())
            }
            None => write!(f, "This list is empty: []"),
        }
    }
}
