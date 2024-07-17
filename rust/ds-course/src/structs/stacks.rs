use std::ptr::NonNull;
use crate::structs::arrays::HeapArray;

pub struct Stack<T> {
    top: NonNull<T>,
    size: usize,
    length: usize,
    data: HeapArray<T>
}

impl<T> Stack<T> {
    pub fn new(size: usize) -> Self {
        let mut arr = HeapArray::with_capacity(size);
        Self {
            top: NonNull::new(arr.as_ptr_mut()).unwrap(),
            size,
            length: 0,
            data: arr
        }
    }

    pub fn push(&mut self, data: T) {
        self.data.push(data);
        self.length += 1;
        let ptr: *mut T;
        if self.length > 1 {
            ptr = unsafe { self.data.as_ptr_mut().add(self.data.get_len() - 1) };
        } else {
            ptr = self.data.as_ptr_mut();
        }
        self.top = NonNull::new(ptr).unwrap();
    }

    pub fn pop(&mut self) -> Option<T> where T: Default {
        if self.length == 0 {
            return None
        }
        let data = self.data.pop().unwrap();
        self.length -= 1;
        let ptr = unsafe { self.data.as_ptr_mut().add(self.data.get_len() - 1) };
        self.top = NonNull::new(ptr).unwrap();
        Some(data)
    }

    pub fn peek(&self, index: usize) -> &T {
        self.data.get(index)
    }

    pub fn get_top(&self) -> NonNull<T> {
        self.top
    }

    pub fn is_empty(&self) -> bool {
        if self.length == 0 {
            return true
        }
        false
    }

    pub fn is_full(&self) -> bool {
        if self.length == self.size {
            return true
        }
        false
    }
}

#[cfg(test)]
mod stack {
    use std::ptr::NonNull;
    use crate::structs::arrays::HeapArray;
    use crate::structs::stacks::Stack;

    #[test]
    fn test_new() {
        let stack: Stack<u8> = Stack::new(5);
        assert_eq!(stack.top, NonNull::new(stack.data.as_ptr_mut()).unwrap(), "Stack top pointer is invalid!");
        assert_eq!(stack.size, 5, "Stack size is invalid!");
        assert_eq!(stack.length, 0, "Stack length is invalid!");
        assert_eq!(stack.data, HeapArray::new(), "Stack array is invalid!");
    }

    #[test]
    fn test_push() {
        let mut stack: Stack<u8> = Stack::new(5);
        stack.push(1);
        assert_eq!(stack.top, NonNull::new(stack.data.as_ptr_mut()).unwrap(), "Stack top pointer is invalid!");
        assert_eq!(stack.size, 5, "Stack size is invalid!");
        assert_eq!(stack.length, 1, "Stack length is invalid!");
        assert_eq!(format!("{}", stack.data), "[1, 0, 0, 0, 0]".to_string(), "Stack array is invalid!");
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<u8> = Stack::new(5);
        assert_eq!(stack.pop(), None, "Empty stack pop value is invalid!");
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2), "Stack pop value is invalid!");
        assert_eq!(stack.top, NonNull::new(stack.data.as_ptr_mut()).unwrap(), "Stack top pointer is invalid!");
        assert_eq!(stack.length, 1, "Stack length is invalid!");
        assert_eq!(format!("{}", stack.data), "[1, 0, 0, 0, 0]".to_string(), "Stack array is invalid!");
    }

    #[test]
    fn test_peek() {
        let mut stack: Stack<u8> = Stack::new(5);
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek(1), &2, "Stack peek value at the index is invalid!");
    }

    #[test]
    fn test_get_top() {
        let mut stack: Stack<u8> = Stack::new(5);
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.get_top(), stack.top, "Stack top pointer is invalid!");
    }

    #[test]
    fn test_is_empty() {
        let mut stack: Stack<u8> = Stack::new(5);
        assert_eq!(stack.is_empty(), true, "Stack should be empty!");
        stack.push(1);
        assert_eq!(stack.is_empty(), false, "Stack should not be empty!");
    }

    #[test]
    fn test_is_full() {
        let mut stack: Stack<u8> = Stack::new(2);
        assert_eq!(stack.is_full(), false, "Stack should not be full!");
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.is_full(), true, "Stack should be full!");
    }
}