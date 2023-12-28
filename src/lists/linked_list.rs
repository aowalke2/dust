use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T: Copy + PartialEq> {
    data: T,
    prev: Option<NonNull<Node<T>>>,
    next: Option<NonNull<Node<T>>>,
}

impl<T: Copy + PartialEq> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            prev: None,
            next: None,
        }
    }
}

pub struct LinkedList<T: Copy + PartialEq> {
    len: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T: Copy + PartialEq> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
            marker: PhantomData,
        }
    }

    pub fn push_front(&mut self, data: T) {
        panic!("Implement")
    }

    pub fn push_back(&mut self, data: T) {
        panic!("Implement")
    }

    pub fn insert(&mut self, index: usize, data: T) {
        panic!("Implement")
    }

    pub fn pop_front(&mut self) -> Option<T> {
        panic!("Implement")
    }

    pub fn pop_back(&mut self) -> Option<T> {
        panic!("Implement")
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        panic!("Implement")
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        panic!("Implement")
    }

    pub fn contains(&self, data: T) -> bool {
        let mut curr = self.head;
        while let Some(curr_ptr) = curr {
            if unsafe { (*curr_ptr.as_ptr()).data == data } {
                return true;
            }

            curr = unsafe { (*curr_ptr.as_ptr()).next };
        }

        false
    }

    pub fn collect(&self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.len);
        let mut curr = self.head;
        while let Some(node_ptr) = curr {
            unsafe {
                let node = &*node_ptr.as_ptr();
                result.push(node.data.clone());
                curr = node.next;
            }
        }

        result
    }
}

impl<T: Copy + PartialEq> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T: Copy + PartialEq + Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T: Copy + PartialEq + Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{} {}", self.data, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.data),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn push_front_works() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        println!("list: {}", list);

        let vec = list.collect();
        assert_eq!(vec, vec![3, 2, 1])
    }

    #[test]
    fn push_back_works() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        println!("list: {}", list);

        let vec = list.collect();
        assert_eq!(vec, vec![1, 2, 3])
    }

    #[test]
    #[should_panic]
    fn insert_panics() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.insert(4, 5)
    }

    #[test]
    fn insert_works() {
        let mut list = LinkedList::new();
        list.insert(0, 1);
        list.insert(1, 3);
        list.insert(1, 2);

        println!("list: {}", list);

        let vec = list.collect();
        assert_eq!(vec, vec![1, 2, 3])
    }

    #[test]
    fn pop_front_returns_none_for_empty_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        println!("list: {}", list);

        assert_eq!(list.pop_front(), None)
    }

    #[test]
    fn pop_front_works() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        println!("list: {}", list);

        match list.pop_front() {
            Some(val) => assert_eq!(val, 3),
            None => panic!("Expected to find {}", 3),
        }

        let vec = list.collect();
        assert_eq!(vec, vec![2, 1])
    }

    #[test]
    fn pop_back_returns_none_for_empty_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        println!("list: {}", list);

        assert_eq!(list.pop_back(), None)
    }

    #[test]
    fn pop_back_works() {
        let mut list = LinkedList::new();
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
    }

    #[test]
    #[should_panic]
    fn remove_panics() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.remove(4);
    }

    #[test]
    fn remove_works() {
        let mut list = LinkedList::new();
        list.insert(0, 1);
        list.insert(1, 3);
        list.insert(1, 2);
        list.insert(3, 4);
        list.insert(4, 5);

        match list.remove(0) {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to find {}", 1),
        }

        match list.remove(3) {
            Some(val) => assert_eq!(val, 5),
            None => panic!("Expected to find {}", 5),
        }

        println!("list: {}", list);

        match list.remove(1) {
            Some(val) => assert_eq!(val, 3),
            None => panic!("Expected to find {}", 3),
        }

        let vec = list.collect();
        assert_eq!(vec, vec![2, 4])
    }

    #[test]
    #[should_panic]
    fn get_panics() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.get(4);
    }

    #[test]
    fn get_works() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        match list.get(1) {
            Some(val) => assert_eq!(*val, 2),
            None => panic!("Expected to find {}", 2),
        }

        let vec = list.collect();
        assert_eq!(vec, vec![1, 2, 3])
    }

    #[test]
    fn contains_works() {
        let mut list = LinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert!(list.contains(2));
        assert!(!list.contains(5));

        let vec = list.collect();
        assert_eq!(vec, vec![1, 2, 3])
    }
}
