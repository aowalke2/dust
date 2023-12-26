use std::fmt::{self, Display, Formatter};
use std::marker::PhantomData;
use std::ptr::NonNull;

struct Node<T: Copy> {
    data: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T: Copy> Node<T> {
    fn new(data: T) -> Self {
        Node {
            data: data,
            next: None,
        }
    }
}

pub struct LinkedList<T: Copy> {
    len: usize,
    head: Option<NonNull<Node<T>>>,
    marker: PhantomData<Box<Node<T>>>,
}

impl<T: Copy> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            len: 0,
            head: None,
            marker: PhantomData,
        }
    }

    pub fn push_front(&mut self, data: T) {
        let mut node = Box::new(Node::new(data));
        node.next = self.head;
        let node_ptr = NonNull::new(Box::into_raw(node));
        self.head = node_ptr;
        self.len += 1;
    }

    pub fn push_back(&mut self, data: T) {
        let node = Box::new(Node::new(data));
        let node_ptr = NonNull::new(Box::into_raw(node));

        match self.head {
            None => self.head = node_ptr,
            Some(mut curr) => {
                while let Some(next_ptr) = unsafe { (*curr.as_ptr()).next } {
                    curr = next_ptr;
                }
                unsafe {
                    (*curr.as_ptr()).next = node_ptr;
                }
            }
        }

        self.len += 1
    }

    pub fn insert(&mut self, index: usize, data: T) {}

    pub fn pop_front(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.head.map(|head_ptr| unsafe {
            let old_head = Box::from_raw(head_ptr.as_ptr());
            self.head = old_head.next;
            self.len = self.len.checked_sub(1).unwrap_or(0);
            old_head.data
        })
    }

    pub fn get(&self, index: i32) -> Option<&T> {
        Self::get_ith_node(self.head, index).map(|ptr| unsafe { &(*ptr.as_ptr()).data })
    }

    fn get_ith_node(node: Option<NonNull<Node<T>>>, index: i32) -> Option<NonNull<Node<T>>> {
        // recursion
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(next_ptr),
                _ => Self::get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
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

impl<T: Copy> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

impl<T: Copy + Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.head {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T: Copy + Display> Display for Node<T> {
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
    fn pop_front_works() {
        let mut list = LinkedList::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        println!("list: {}", list);

        match list.pop_front() {
            Some(val) => assert_eq!(val, 3),
            None => panic!("Expected to find {} a index 0", 3),
        }

        let vec = list.collect();
        assert_eq!(vec, vec![2, 1])
    }

    #[test]
    fn pop_front_returns_none_for_empty_list() {
        let mut list: LinkedList<i32> = LinkedList::new();
        println!("list: {}", list);

        assert_eq!(list.pop_front(), None)
    }
}
