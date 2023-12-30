use std::alloc::{self, Layout};
use std::mem;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ptr::{self, NonNull};

struct RawVec<T> {
    ptr: NonNull<T>,
    cap: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        let cap = if mem::size_of::<T>() == 0 {
            usize::MAX
        } else {
            0
        };

        RawVec {
            ptr: NonNull::dangling(),
            cap,
        }
    }

    fn grow(&mut self) {
        assert!(mem::size_of::<T>() != 0, "capacity overflow");

        let (new_cap, new_layout) = if self.cap == 0 {
            (1, Layout::array::<T>(1).unwrap())
        } else {
            let new_cap = 2 * self.cap;

            let new_layout = Layout::array::<T>(new_cap).unwrap();
            (new_cap, new_layout)
        };

        assert!(
            new_layout.size() <= isize::MAX as usize,
            "Allocation too large"
        );

        let new_ptr = if self.cap == 0 {
            unsafe { alloc::alloc(new_layout) }
        } else {
            let old_layout = Layout::array::<T>(self.cap).unwrap();
            let old_ptr = self.ptr.as_ptr() as *mut u8;
            unsafe { alloc::realloc(old_ptr, old_layout, new_layout.size()) }
        };

        self.ptr = match NonNull::new(new_ptr as *mut T) {
            Some(p) => p,
            None => alloc::handle_alloc_error(new_layout),
        };
        self.cap = new_cap;
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        let elem_size = mem::size_of::<T>();

        if self.cap != 0 && elem_size != 0 {
            unsafe {
                alloc::dealloc(
                    self.ptr.as_ptr() as *mut u8,
                    Layout::array::<T>(self.cap).unwrap(),
                );
            }
        }
    }
}
pub struct Vec<T> {
    buffer: RawVec<T>,
    len: usize,
}

impl<T> Vec<T> {
    fn ptr(&self) -> *mut T {
        self.buffer.ptr.as_ptr()
    }

    fn cap(&self) -> usize {
        self.buffer.cap
    }

    pub fn new() -> Self {
        Vec {
            buffer: RawVec::new(),
            len: 0,
        }
    }

    pub fn push(&mut self, elem: T) {
        if self.len == self.cap() {
            self.buffer.grow();
        }

        unsafe {
            ptr::write(self.ptr().add(self.len), elem);
        }
        self.len += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        unsafe { Some(ptr::read(self.ptr().add(self.len))) }
    }

    pub fn insert(&mut self, index: usize, elem: T) {
        assert!(index <= self.len, "index out of bounds");
        if self.cap() == self.len {
            self.buffer.grow();
        }

        //shift all the elements from [i .. len] to [i+1 .. len+1] using the old len.
        unsafe {
            ptr::copy(
                self.ptr().add(index),
                self.ptr().add(index + 1),
                self.len - index,
            );
            ptr::write(self.ptr().add(index), elem);
            self.len += 1;
        }
    }

    pub fn remove(&mut self, index: usize) -> T {
        assert!(index < self.len, "index out of bounds");

        //shift all the elements from [i+1 .. len + 1] to [i .. len] using the new len.
        unsafe {
            self.len -= 1;
            let result = ptr::read(self.ptr().add(index));
            ptr::copy(
                self.ptr().add(index + 1),
                self.ptr().add(index),
                self.len - index,
            );
            result
        }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        while let Some(_) = self.pop() {}
        // deallocation is handled by RawVec
    }
}

impl<T> Deref for Vec<T> {
    type Target = [T];
    fn deref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.ptr(), self.len) }
    }
}

impl<T> DerefMut for Vec<T> {
    fn deref_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.ptr(), self.len) }
    }
}

#[cfg(test)]
mod tests {
    use super::Vec;

    #[test]
    fn push_works() {
        let mut list = Vec::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list[0], 1);
        assert_eq!(list[1], 2);
        assert_eq!(list[2], 3);
    }

    #[test]
    fn pop_returns_none_for_empty_vec() {
        let mut list: Vec<i32> = Vec::new();

        assert_eq!(list.pop(), None);
    }

    #[test]
    fn pop_works() {
        let mut list = Vec::new();
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
    }

    #[test]
    fn insert_works() {
        let mut list = Vec::new();
        list.push(1);
        list.push(2);
        list.push(3);

        list.insert(1, 4);

        assert_eq!(list[0], 1);
        assert_eq!(list[1], 4);
        assert_eq!(list[2], 2);
        assert_eq!(list[3], 3);
    }

    #[test]
    fn delete_works() {
        let mut list = Vec::new();
        list.push(1);
        list.push(2);
        list.push(3);

        list.remove(1);

        assert_eq!(list[0], 1);
        assert_eq!(list[1], 3);
    }
}
