use std::collections::LinkedList;
use std::fmt::{self, Display};

pub struct LinkedListQueue<T: Copy + PartialEq> {
    buffer: LinkedList<T>,
}

impl<T: Copy + PartialEq> LinkedListQueue<T> {
    pub fn new() -> Self {
        LinkedListQueue {
            buffer: LinkedList::new(),
        }
    }

    pub fn enqueue(&mut self, elt: T) {
        self.buffer.push_back(elt)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.buffer.pop_front()
    }

    pub fn top(&self) -> Option<&T> {
        self.buffer.front()
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

impl<T: Copy + PartialEq + Display> Display for LinkedListQueue<T> {
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
    use super::LinkedListQueue;

    #[test]
    fn enqueue_works() {
        let mut queue = LinkedListQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        println!("queue: {}", queue);

        let vec = queue.collect();
        assert_eq!(vec, vec![1, 2, 3]);
        assert_eq!(queue.len(), 3);
    }

    #[test]
    fn dequeue_returns_none_for_empty_list() {
        let mut queue: LinkedListQueue<i32> = LinkedListQueue::new();
        println!("list: {}", queue);

        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn dequeue_works() {
        let mut queue = LinkedListQueue::new();
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        println!("stack: {}", queue);

        match queue.dequeue() {
            Some(val) => assert_eq!(val, 1),
            None => panic!("Expected to find {}", 1),
        }

        let vec = queue.collect();
        assert_eq!(vec, vec![2, 3]);

        match queue.dequeue() {
            Some(val) => assert_eq!(val, 2),
            None => panic!("Expected to find {}", 2),
        }

        match queue.dequeue() {
            Some(val) => assert_eq!(val, 3),
            None => panic!("Expected to find {}", 3),
        }

        assert_eq!(queue.len(), 0);
    }
}
