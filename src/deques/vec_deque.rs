use std::fmt::{self, Display};

pub struct VecDeque<T: Copy + PartialEq> {
    buffer: Vec<T>,
}

impl<T: Copy + PartialEq> VecDeque<T> {
    pub fn new() -> Self {
        VecDeque { buffer: Vec::new() }
    }

    pub fn push_front(&mut self, element: T) {
        let mut new_buffer = vec![element];
        new_buffer.append(self.buffer.as_mut());
        self.buffer = new_buffer;
    }

    pub fn push_back(&mut self, element: T) {
        self.buffer.push(element)
    }

    pub fn insert(&mut self, index: usize, element: T) {
        self.buffer.insert(index, element)
    }

    pub fn pop_front(&mut self) -> Option<T> {
        if self.buffer.len() == 0 {
            return None;
        }

        let result = self.buffer[0];
        self.buffer = self.buffer[1..].to_vec();
        Some(result)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.buffer.pop()
    }

    pub fn remove(&mut self, index: usize) -> T {
        self.buffer.remove(index)
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

impl<T: Copy + PartialEq + Display> Display for VecDeque<T> {
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
    use super::VecDeque;

    #[test]
    fn push_front_works() {
        let mut list = VecDeque::new();
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
        let mut list = VecDeque::new();
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
        let mut list = VecDeque::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.insert(4, 5);
    }

    #[test]
    fn insert_works() {
        let mut list = VecDeque::new();
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
        let mut list: VecDeque<i32> = VecDeque::new();
        println!("list: {}", list);

        assert_eq!(list.pop_front(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_front_works() {
        let mut list = VecDeque::new();
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
        let mut list: VecDeque<i32> = VecDeque::new();
        println!("list: {}", list);

        assert_eq!(list.pop_back(), None);
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn pop_back_works() {
        let mut list = VecDeque::new();
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
        let mut list = VecDeque::new();
        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        list.remove(4);
    }

    #[test]
    fn remove_works() {
        let mut list = VecDeque::new();
        list.insert(0, 1);
        list.insert(1, 3);
        list.insert(1, 2);
        list.insert(3, 4);
        list.insert(4, 5);

        assert_eq!(list.remove(0), 1);
        assert_eq!(list.remove(3), 5);
        assert_eq!(list.remove(1), 3);

        println!("list: {}", list);

        let vec = list.collect();
        assert_eq!(vec, vec![2, 4]);
        assert_eq!(list.len(), 2);
    }
}
