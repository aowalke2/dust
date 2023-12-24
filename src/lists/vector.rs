pub struct Vector<T> {
    len: usize,
    array: [T; 0],
}

impl<T> Vector<T> {
    pub fn new() -> Self {
        Vector { array: [], len: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result: Vector<i32> = Vector::new();
    }
}
