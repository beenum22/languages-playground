use crate::structs::arrays::HeapArray;
use crate::structs::linked_lists::{LinkedList, Node};
use crate::structs::smart_ptrs::AtomicReferenceCounter;
use std::ptr::NonNull;

pub struct Stack<T> {
    top: NonNull<T>,
    size: usize,
    length: usize,
    data: HeapArray<T>, // It would be better to use an Array on Stack since it's fixed size.
}

impl<T> Stack<T> {
    pub fn new(size: usize) -> Self {
        let arr = HeapArray::with_capacity(size);
        Self {
            top: NonNull::new(arr.as_ptr_mut()).unwrap(),
            size,
            length: 0,
            data: arr,
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

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        let data = self.data.pop().unwrap();
        self.length -= 1;
        let ptr = unsafe { self.data.as_ptr_mut().add(self.data.get_len()) };
        self.top = NonNull::new(ptr).unwrap();
        Some(data)
    }

    pub fn peek(&self) -> Option<&T> {
        if self.length == 0 {
            return None;
        }
        Some(self.data.get(self.data.get_len() - 1))
    }

    pub fn get(&self, index: usize) -> &T {
        self.data.get(index)
    }

    pub fn get_top(&self) -> NonNull<T> {
        self.top
    }

    pub fn is_empty(&self) -> bool {
        if self.length == 0 {
            return true;
        }
        false
    }

    pub fn is_full(&self) -> bool {
        if self.length == self.size {
            return true;
        }
        false
    }
}

pub struct DynamicStack<T> {
    top: Option<AtomicReferenceCounter<Node<T>>>,
    length: usize,
    data: LinkedList<T>,
}

impl<T> DynamicStack<T> {
    pub fn new() -> Self {
        let ll = LinkedList::new();
        Self {
            top: None,
            length: 0,
            data: ll,
        }
    }

    // Time Complexity is O(1)
    pub fn push(&mut self, data: T) {
        self.data.push_front(data);
        self.length += 1;
        self.top = Some(self.data.head_as_ref().unwrap().clone());
    }

    // Time Complexity is O(1)
    pub fn pop(&mut self) -> Option<T>
    where
        T: Copy,
    {
        if self.length == 0 {
            return None;
        }
        let data = self.data.pop_front().unwrap();
        self.length -= 1;
        self.top = Some(self.data.head_as_ref().unwrap().clone());
        Some(data)
    }

    // TODO: Use to get top value only and have separate method for get with index.
    // Time Complexity is min: O(1), max: O(n)
    pub fn peek(&self, index: usize) -> Option<&T> {
        Some(self.data.peek(index).expect("Index out of bounds!"))
    }

    pub fn get_top(&self) -> Option<&AtomicReferenceCounter<Node<T>>> {
        self.top.as_ref()
    }

    pub fn is_empty(&self) -> bool {
        if self.length == 0 {
            return true;
        }
        false
    }

    pub fn is_full(&self) -> bool {
        // if self.length == self.size {
        //     return true
        // }
        false
    }
}

#[cfg(test)]
mod stack {
    use crate::structs::arrays::HeapArray;
    use crate::structs::stacks::Stack;
    use std::ptr::NonNull;

    #[test]
    fn test_new() {
        let stack: Stack<u8> = Stack::new(5);
        assert_eq!(
            stack.top,
            NonNull::new(stack.data.as_ptr_mut()).unwrap(),
            "Stack top pointer is invalid!"
        );
        assert_eq!(stack.size, 5, "Stack size is invalid!");
        assert_eq!(stack.length, 0, "Stack length is invalid!");
        assert_eq!(stack.data, HeapArray::new(), "Stack array is invalid!");
    }

    #[test]
    fn test_push() {
        let mut stack: Stack<u8> = Stack::new(5);
        stack.push(1);
        assert_eq!(
            stack.top,
            NonNull::new(stack.data.as_ptr_mut()).unwrap(),
            "Stack top pointer is invalid!"
        );
        assert_eq!(stack.size, 5, "Stack size is invalid!");
        assert_eq!(stack.length, 1, "Stack length is invalid!");
        assert_eq!(
            format!("{}", stack.data),
            "[1]".to_string(),
            "Stack array is invalid!"
        );
        stack.push(2);
        assert_eq!(
            format!("{}", stack.data),
            "[1, 2]".to_string(),
            "Stack array is invalid!"
        );
    }

    #[test]
    fn test_pop() {
        let mut stack: Stack<u8> = Stack::new(5);
        assert_eq!(stack.pop(), None, "Empty stack pop value is invalid!");
        stack.push(1);
        stack.push(2);
        stack.push(3);
        assert_eq!(stack.pop(), Some(3), "Stack pop value is invalid!");
        unsafe {
            assert_eq!(
                stack.top,
                NonNull::new(stack.data.as_ptr_mut().add(2)).unwrap(),
                "Stack top pointer is invalid!"
            );
        }
        assert_eq!(stack.length, 2, "Stack length is invalid!");
        assert_eq!(
            format!("{}", stack.data),
            "[1, 2]".to_string(),
            "Stack array is invalid!"
        );
        stack.pop();
        stack.pop();
        assert_eq!(stack.pop(), None, "Stack pop value is invalid!");
        assert_eq!(stack.length, 0, "Stack length is invalid!");
        assert_eq!(
            stack.top,
            NonNull::new(stack.data.as_ptr_mut()).unwrap(),
            "Stack top pointer is invalid!"
        );
    }

    #[test]
    fn test_peek() {
        let mut stack: Stack<u8> = Stack::new(5);
        assert_eq!(stack.peek(), None, "Stack peek value is invalid!");
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.peek(), Some(&2), "Stack peek value is invalid!");
    }

    #[test]
    fn test_get() {
        let mut stack: Stack<u8> = Stack::new(5);
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.get(1), &2, "Stack get value at the index is invalid!");
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

mod dynamic_stack {
    use crate::structs::stacks::DynamicStack;

    #[test]
    fn test_new() {
        let stack: DynamicStack<u8> = DynamicStack::new();
        assert_eq!(stack.top, None, "Stack top pointer is invalid!");
        assert_eq!(stack.length, 0, "Stack length is invalid!");
        assert_eq!(
            format!("{}", stack.data),
            "None".to_string(),
            "Stack array is invalid!"
        );
    }

    #[test]
    fn test_push() {
        let mut stack: DynamicStack<u8> = DynamicStack::new();
        stack.push(1);
        assert_eq!(
            stack.top,
            Some(stack.data.head_as_ref().unwrap().clone()),
            "Stack top pointer is invalid!"
        );
        assert_eq!(stack.length, 1, "Stack length is invalid!");
        assert_eq!(
            format!("{}", stack.data),
            "1".to_string(),
            "Stack array is invalid!"
        );
    }

    #[test]
    fn test_pop() {
        let mut stack: DynamicStack<u8> = DynamicStack::new();
        assert_eq!(stack.pop(), None, "Empty stack pop value is invalid!");
        stack.push(1);
        stack.push(2);
        assert_eq!(stack.pop(), Some(2), "Stack pop value is invalid!");
        assert_eq!(
            stack.top,
            Some(stack.data.head_as_ref().unwrap().clone()),
            "Stack top pointer is invalid!"
        );
        assert_eq!(stack.length, 1, "Stack length is invalid!");
        assert_eq!(
            format!("{}", stack.data),
            "1".to_string(),
            "Stack array is invalid!"
        );
    }

    #[test]
    fn test_peek() {
        let mut stack: DynamicStack<u8> = DynamicStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(
            stack.peek(0),
            Some(&2u8),
            "Stack peek value at index 0 is invalid!"
        );
        assert_eq!(
            stack.peek(1),
            Some(&1u8),
            "Stack peek value at index 1 is invalid!"
        );
    }

    #[test]
    fn test_get_top() {
        let mut stack: DynamicStack<u8> = DynamicStack::new();
        stack.push(1);
        stack.push(2);
        assert_eq!(
            stack.get_top(),
            stack.data.head_as_ref(),
            "Stack top pointer is invalid!"
        );
    }

    #[test]
    fn test_is_empty() {
        let mut stack: DynamicStack<u8> = DynamicStack::new();
        assert_eq!(stack.is_empty(), true, "Stack should be empty!");
        stack.push(1);
        assert_eq!(stack.is_empty(), false, "Stack should not be empty!");
    }

    // #[test]
    fn test_is_full() {
        let mut stack: DynamicStack<u8> = DynamicStack::new();
        assert_eq!(stack.is_full(), false, "Stack should not be full!");
        stack.push(1);
        stack.push(2);
        assert_eq!(
            stack.is_full(),
            false,
            "Stack should never be full since it's dynamic!"
        );
    }
}
