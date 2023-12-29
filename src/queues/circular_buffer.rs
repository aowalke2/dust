use std::fmt::{self, Display};

pub struct CircularBuffer<T: Copy + PartialEq> {
    buffer: Vec<Option<T>>,
    len: usize,
    head: usize,
    tail: usize,
}

impl<T: Copy + PartialEq> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: vec![None; capacity],
            len: 0,
            head: 0,
            tail: 0,
        }
    }

    pub fn enqueue(&mut self, elt: T) {
        if self.is_full() {
            panic!("Buffer is full. Item not added")
        }

        self.buffer[self.tail] = Some(elt);
        self.tail = (self.tail + 1) % self.buffer.len();
        self.len += 1;
    }

    pub fn dequeue(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let element = self.buffer[self.head].take();
        self.head = (self.head + 1) % self.buffer.len();
        self.len -= 1;
        element
    }

    pub fn top(&self) -> Option<&T> {
        self.buffer[self.head].as_ref()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn is_full(&self) -> bool {
        self.len == self.buffer.len()
    }

    pub fn collect(&self) -> Vec<T> {
        self.buffer
            .iter()
            .filter(|elm| elm.is_some())
            .map(|elm| elm.unwrap())
            .collect::<Vec<T>>()
    }
}

impl<T: Copy + PartialEq + Display> Display for CircularBuffer<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut iter = self
            .buffer
            .iter()
            .filter(|x| x.is_some())
            .map(|x| x.unwrap());

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
    use super::CircularBuffer;

    #[test]
    fn enqueue_works() {
        let mut queue = CircularBuffer::new(3);
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
        let mut queue: CircularBuffer<i32> = CircularBuffer::new(3);
        println!("list: {}", queue);

        assert_eq!(queue.dequeue(), None);
        assert_eq!(queue.len(), 0);
    }

    #[test]
    fn dequeue_works() {
        let mut queue = CircularBuffer::new(3);
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
