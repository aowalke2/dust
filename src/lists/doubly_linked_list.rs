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

pub struct DoublyLinkedList<T: Copy + PartialEq> {
    len: usize,
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T: Copy + PartialEq> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            tail: None,
            marker: PhantomData,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let mut node = Box::new(Node::new(data));
        node.next = self.head;

        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.head {
            None => self.tail = node_ptr,
            Some(head_ptr) => unsafe { (*head_ptr.as_ptr()).prev = node_ptr },
        }

        self.head = node_ptr;
        self.len += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let mut node = Box::new(Node::new(data));
        node.prev = self.tail;

        let node_ptr = NonNull::new(Box::into_raw(node));
        match self.tail {
            None => self.head = node_ptr,
            Some(tail_ptr) => unsafe { (*tail_ptr.as_ptr()).next = node_ptr },
        }

        self.tail = node_ptr;
        self.len += 1;
    }

    pub fn insert(&mut self, index: usize, data: T) {
        if self.len < index {
            panic!("Index out of bounds")
        }

        if index == 0 || self.head.is_none() {
            self.push_front(data);
            return;
        }

        if self.len == index {
            self.push_back(data);
            return;
        }

        // Optimization to based on index distance from each end
        if index + 1 <= self.len / 2 {
            let mut curr = self.head;
            let mut count = 0;

            while let Some(curr_ptr) = curr {
                if count == index {
                    let mut node = Box::new(Node::new(data));

                    unsafe {
                        node.prev = (*curr_ptr.as_ptr()).prev;
                        node.next = Some(curr_ptr);

                        if let Some(prev_ptr) = (*curr_ptr.as_ptr()).prev {
                            let node_ptr = NonNull::new(Box::into_raw(node));
                            (*prev_ptr.as_ptr()).next = node_ptr;
                            (*curr_ptr.as_ptr()).prev = node_ptr;
                        }
                    };

                    self.len += 1;
                    return;
                }

                count += 1;
                curr = unsafe { (*curr_ptr.as_ptr()).next };
            }
        } else {
            let mut curr = self.tail;
            let mut count = self.len - 1;

            while let Some(curr_ptr) = curr {
                if count == index {
                    let mut node = Box::new(Node::new(data));

                    unsafe {
                        node.prev = (*curr_ptr.as_ptr()).prev;
                        node.next = Some(curr_ptr);

                        if let Some(prev_ptr) = (*curr_ptr.as_ptr()).prev {
                            let node_ptr = NonNull::new(Box::into_raw(node));
                            (*prev_ptr.as_ptr()).next = node_ptr;
                            (*curr_ptr.as_ptr()).prev = node_ptr;
                        }
                    };

                    self.len += 1;
                    return;
                }

                count -= 1;
                curr = unsafe { (*curr_ptr.as_ptr()).prev };
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());
            match old_head.next {
                None => self.tail = None,
                Some(mut next_ptr) => next_ptr.as_mut().prev = None,
            }
            self.head = old_head.next;
            self.len -= 1;
            old_head.data
        })
    }

    pub fn pop_back(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.tail.map(|tail_ptr| unsafe {
            let old_tail = Box::from_raw(tail_ptr.as_ptr());
            match old_tail.prev {
                None => self.head = None,
                Some(mut prev_ptr) => prev_ptr.as_mut().next = None,
            }
            self.tail = old_tail.prev;
            self.len -= 1;
            old_tail.data
        })
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if self.len <= index {
            panic!("Index out of bounds")
        }

        if index == 0 || self.head.is_none() {
            return self.pop_front();
        }

        if self.len - 1 == index {
            return self.pop_back();
        }

        if index + 1 <= self.len / 2 {
            let mut curr = self.head;
            let mut count = 0;

            while let Some(curr_ptr) = curr {
                if count == index {
                    unsafe {
                        let old_node = Box::from_raw(curr_ptr.as_ptr());
                        if let Some(mut prev) = old_node.prev {
                            prev.as_mut().next = old_node.next;
                        };

                        if let Some(mut next) = old_node.next {
                            next.as_mut().prev = old_node.prev;
                        }

                        self.len -= 1;
                        return Some(old_node.data);
                    };
                }

                count += 1;
                curr = unsafe { (*curr_ptr.as_ptr()).next };
            }
        } else {
            let mut curr = self.tail;
            let mut count = self.len - 1;

            while let Some(curr_ptr) = curr {
                if count == index {
                    unsafe {
                        let old_node = Box::from_raw(curr_ptr.as_ptr());
                        if let Some(mut prev) = old_node.prev {
                            prev.as_mut().next = old_node.next;
                        };

                        if let Some(mut next) = old_node.next {
                            next.as_mut().prev = old_node.prev;
                        }

                        self.len -= 1;
                        return Some(old_node.data);
                    };
                }

                count -= 1;
                curr = unsafe { (*curr_ptr.as_ptr()).prev };
            }
        }

        None
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        if self.len <= index {
            panic!("Index out of bounds")
        }

        if index + 1 <= self.len / 2 {
            let mut curr = self.head;
            let mut count = 0;

            while let Some(curr_ptr) = curr {
                if count == index {
                    return Some(unsafe { &(*curr_ptr.as_ptr()).data });
                }
                count += 1;
                curr = unsafe { (*curr_ptr.as_ptr()).next };
            }
        } else {
            let mut curr = self.tail;
            let mut count = self.len - 1;

            while let Some(curr_ptr) = curr {
                if count == index {
                    return Some(unsafe { &(*curr_ptr.as_ptr()).data });
                }
                count -= 1;
                curr = unsafe { (*curr_ptr.as_ptr()).prev };
            }
        }

        None
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

impl<T: Copy + PartialEq> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T: Copy + PartialEq + Display> Display for DoublyLinkedList<T> {
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
    use super::DoublyLinkedList;

    #[test]
    fn push_front_works() {
        let mut list = DoublyLinkedList::new();
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
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);
        println!("list: {}", list);

        let vec = list.collect();
        assert_eq!(vec, vec![1, 2, 3]);
        assert_eq!(list.len(), 3);
    }

    #[test]
    #[should_panic]
    fn insert_panics() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.insert(4, 5);
    }

    #[test]
    fn insert_works() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(1);
        list.push_back(1);
        list.push_back(1);
        list.push_back(1);
        list.insert(0, 1);
        list.insert(6, 2);
        list.insert(2, 2);
        list.insert(6, 3);

        println!("list: {}", list);

        let vec = list.collect();
        assert_eq!(vec, vec![1, 1, 2, 1, 1, 1, 3, 1, 2]);
        assert_eq!(list.len(), 9);
    }

    #[test]
    fn pop_front_returns_none_for_empty_list() {
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        println!("list: {}", list);

        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_front_works() {
        let mut list = DoublyLinkedList::new();
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
        let mut list: DoublyLinkedList<i32> = DoublyLinkedList::new();
        println!("list: {}", list);

        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_back_works() {
        let mut list = DoublyLinkedList::new();
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

    #[test]
    #[should_panic]
    fn remove_panics() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.remove(4);
    }

    #[test]
    fn remove_works() {
        let mut list = DoublyLinkedList::new();
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
        assert_eq!(vec, vec![2, 4]);
        assert_eq!(list.len(), 2);
    }

    #[test]
    #[should_panic]
    fn get_panics() {
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.get(4);
    }

    #[test]
    fn get_works() {
        let mut list = DoublyLinkedList::new();
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
        let mut list = DoublyLinkedList::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert!(list.contains(2));
        assert!(!list.contains(5));

        let vec = list.collect();
        assert_eq!(vec, vec![1, 2, 3])
    }
}
