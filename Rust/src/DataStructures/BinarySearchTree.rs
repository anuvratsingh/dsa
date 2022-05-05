// Requirements
//    1. Store IoT deice objects (IP address, numerical name and Type).
//    2. Retrieve IoT objects by numerical name.
//    3. Iterate over Iot objects.

// Access, Search, Insertion and Deletion are Avg O(log(n))
// Access, Search, Insertion and Deletion are Worst O(n)

// Positives
//    1. Simple implementation.
//    2. Efficient and fast search.
//    3. Traversal allows for different orderings.
//    4. Great for large amounts of unsorted data.

// Negatives
//    1. Worst-case performance is that of a linked list.
//    2. Unbalanced trees are easy to create by accident.
//    3. Unbalanced trees cannot be repaired.
//    4. Recursive algorithms can overflow on unbalanced trees.
use std::mem;

#[derive(Clone, Debug)]
pub struct IoTDevice<I, A> {
  pub id: I,
  pub address: A,
  pub path: A,
}

impl<I, A> IoTDevice<I, A> {
  pub fn new(id: I, address: impl Into<A>, path: impl Into<A>) -> IoTDevice<I, A> {
    IoTDevice {
      id,
      address: address.into(),
      path: path.into(),
    }
  }
}

impl<I: PartialEq, A: PartialEq> PartialEq for IoTDevice<I, A> {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id && self.address == other.address
  }
}

type Tree<I, A> = Option<Box<Node<I, A>>>;
struct Node<I, A> {
  pub dev: IoTDevice<I, A>,
  left: Tree<I, A>,
  right: Tree<I, A>,
}

impl<I, A> Node<I, A> {
  fn new(dev: IoTDevice<I, A>) -> Tree<I, A> {
    Some(Box::new(Node {
      dev,
      left: None,
      right: None,
    }))
  }
}

pub struct DeviceRegistry<I, A> {
  root: Tree<I, A>,
    pub length: usize,
}

impl<I: PartialOrd + Clone, A: Clone> DeviceRegistry<I, A> {
    pub fn new_empty() -> DeviceRegistry<I, A> {
        DeviceRegistry {
            root: None,
            length: 0,
        }
    }

    pub fn add(&mut self, device: IoTDevice<I, A>) {
        self.length += 1;
        let root = mem::replace(&mut self.root, None);
        self.root = self.add_rec(root, device);
    }

    fn add_rec(&self, node: Tree<I, A>, device: IoTDevice<I, A>) -> Tree<I, A> {
        match node {
            Some(mut n) => {
                if n.dev.id <= device.id {
                    n.left = self.add_rec(n.left, device);
                } else {
                    n.right = self.add_rec(n.right, device);
                }
                Some(n)
            }
            None => Node::new(device),
        }
    }

    pub fn find(&self, id: I) -> Option<IoTDevice<I, A>> {
        self.find_r(&self.root, id)
    }

    fn find_r(&self, node: &Tree<I, A>, id: I) -> Option<IoTDevice<I, A>> {
        match node {
            Some(n) => {
                if n.dev.id == id {
                    Some(n.dev.clone())
                } else if n.dev.id < id {
                    self.find_r(&n.left, id)
                } else {
                    self.find_r(&n.right, id)
                }
            }
            None => None,
        }
    }

    pub fn walk(&self, callback: impl Fn(&IoTDevice<I, A>) -> ()) {
        self.walk_in_order(&self.root, &callback)
    }

    fn walk_in_order(&self, node: &Tree<I, A>, callback: &impl Fn(&IoTDevice<I, A>)) {
        if let Some(n) = node {
            self.walk_in_order(&n.left, callback);
            callback(&n.dev);
            self.walk_in_order(&n.right, callback);
        }
    }
}
