use std::cmp;

type Node<T> = Option<T>;
const MIN_SIZE: usize = 8;

// Requirements
//    1. Save a transaction's timestamp in a list.
//    2. Access the elements quickly by index, in any order.
//    3. Iterate the items in the order they were saved.

// Access & Search are Avg O(1)
// Insertion and Deletion are Avg O(n)

// Positives
//    1. Speed: arrays/slices make things really fast.
//    2. Simple and fast element access.
//    3. Clear ownership structures.
//    4. Fast append and iteration.
//    5. Very CPU cache-friendly.
// Negatives
//    1. Operations other than append will require to shift elements.
//    2. Growth strategy is not memory-efficient.
//    3. A single large chunk of memory is required
//    Size is limited by usize type, which differs from platform to
//    platform.
//    4. Growth speed decreases with list size.

pub struct TimestampSaver<T> {
    buf: Box<[Node<T>]>,
    cap: usize,
    pub length: usize,
}

impl<T: Copy> TimestampSaver<T> {
    pub fn new_empty() -> Self {
        Self {
            buf: Box::new([None; MIN_SIZE]),
            cap: MIN_SIZE,
            length: 0,
        }
    }
    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        let mut new_cap = old_cap + (old_cap >> 1);

        new_cap = cmp::max(new_cap, min_cap);
        new_cap = cmp::min(new_cap, usize::MAX);

        let current = self.buf.clone();
        self.cap = new_cap;
        self.buf = vec![None; new_cap].into_boxed_slice();
        self.buf[..current.len()].clone_from_slice(&current);
    }

    pub fn at(&mut self, index: usize) -> Option<T> {
        if self.length > index {
            self.buf[index]
        } else {
            None
        }
    }

    pub fn append(&mut self, value: T) {
        if self.cap == self.length {
            self.grow(self.length + 1)
        }
        self.buf[self.length] = Some(value);
        self.length += 1;
    }
}

impl<T: Copy> IntoIterator for TimestampSaver<T> {
    type Item = T;
    type IntoIter = ListIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        ListIterator::new(0, self.buf)
    }
}

pub struct ListIterator<T> {
    current: usize,
    data: Box<[Node<T>]>,
}

impl<T> ListIterator<T> {
    fn new(index: usize, buf: Box<[Node<T>]>) -> ListIterator<T> {
        ListIterator {
            current: index,
            data: buf,
        }
    }
}

impl<T: Copy> Iterator for ListIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            self.current += 1;
            item
        } else {
            None
        }
    }
}

impl<T: Copy> DoubleEndedIterator for ListIterator<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            if self.current == 0 {
                self.current = self.data.len() - 1;
            } else {
                self.current -= 1;
            }
            item
        } else {
            None
        }
    }
}
