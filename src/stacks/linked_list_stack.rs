use std::collections::LinkedList;
use std::fmt::{self, Display};

pub struct LinkedListStack<T: Copy + PartialEq> {
    buffer: LinkedList<T>,
}

impl<T: Copy + PartialEq> LinkedListStack<T> {
    pub fn new() -> Self {
        LinkedListStack {
            buffer: LinkedList::new(),
        }
    }

    pub fn push(&mut self, elt: T) {
        self.buffer.push_back(elt)
    }

    pub fn pop(&mut self) -> Option<T> {
        self.buffer.pop_back()
    }

    pub fn top(&self) -> Option<&T> {
        self.buffer.back()
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

impl<T: Copy + PartialEq + Display> Display for LinkedListStack<T> {
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
    use super::LinkedListStack;

    #[test]
    fn push_works() {
        let mut stack = LinkedListStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        println!("stack: {}", stack);

        let vec = stack.collect();
        assert_eq!(vec, vec![1, 2, 3]);
        assert_eq!(stack.len(), 3);
    }

    #[test]
    fn pop_returns_none_for_empty_list() {
        let mut stack: LinkedListStack<i32> = LinkedListStack::new();
        println!("stack: {}", stack);

        assert_eq!(stack.pop(), None);
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn pop_works() {
        let mut stack = LinkedListStack::new();
        stack.push(1);
        stack.push(2);
        stack.push(3);
        println!("stack: {}", stack);

        match stack.pop() {
            Some(val) => assert_eq!(val, 3),
            None => panic!("Expected to find {}", 3),
        }

        let vec = stack.collect();
        assert_eq!(vec, vec![1, 2]);

        match stack.pop() {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to find {}", 2),
        }

        match stack.pop() {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to find {}", 1),
        }

        assert_eq!(stack.len(), 0);
    }
}
