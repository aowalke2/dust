use std::collections::LinkedList;
use std::fmt::{self, Display};

pub struct LinkedListDeque<T: Copy + PartialEq> {
    buffer: LinkedList<T>,
}

impl<T: Copy + PartialEq> LinkedListDeque<T> {
    pub fn new() -> Self {
        LinkedListDeque {
            buffer: LinkedList::new(),
        }
    }

    pub fn push_front(&mut self, elt: T) {
        self.buffer.push_front(elt)
    }

    pub fn push_back(&mut self, elt: T) {
        self.buffer.push_back(elt)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.buffer.pop_front()
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.buffer.pop_back()
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }

    pub fn collect(&self) -> Vec<T> {
        self.buffer
            .iter()
            .map(|elm| elm.clone())
            .collect::<Vec<T>>()
    }
}

impl<T: Copy + PartialEq + Display> Display for LinkedListDeque<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut iter = self.buffer.iter();

        if let Some(value) = iter.next() {
            write!(f, "{}", value)?;

            for value in iter {
                write!(f, "{}", value)?;
            }
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedListDeque;

    #[test]
    fn push_front_works() {
        let mut list = LinkedListDeque::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        println!("list: {}", list);

        let vec = list.collect();
        assert_eq!(vec, vec![3, 2, 1]);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn push_back_works() {
        let mut list = LinkedListDeque::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        println!("list: {}", list);

        let vec = list.collect();
        assert_eq!(vec, vec![1, 2, 3]);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn pop_front_returns_none_for_empty_list() {
        let mut list: LinkedListDeque<i32> = LinkedListDeque::new();
        println!("list: {}", list);

        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_front_works() {
        let mut list = LinkedListDeque::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        println!("list: {}", list);

        match list.pop_front() {
            Some(val) => assert_eq!(val, 3),
            None => panic!("Expected to find {}", 3),
        }

        let vec = list.collect();
        assert_eq!(vec, vec![2, 1]);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn pop_back_returns_none_for_empty_list() {
        let mut list: LinkedListDeque<i32> = LinkedListDeque::new();
        println!("list: {}", list);

        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_back_works() {
        let mut list = LinkedListDeque::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        println!("list: {}", list);

        match list.pop_back() {
            Some(val) => assert_eq!(val, 3),
            None => panic!("Expected to find {}", 3),
        }

        let vec = list.collect();
        assert_eq!(vec, vec![1, 2]);

        match list.pop_back() {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to find {}", 2),
        }

        match list.pop_back() {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to find {}", 1),
        }

        assert_eq!(list.len(), 0);
    }
}
