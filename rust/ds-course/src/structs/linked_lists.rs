use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Deref};
use crate::structs::smart_ptr::{SmartPointer};

type Link<T> = Option<SmartPointer<Node<T>>>;

#[derive(Debug)]
#[derive(PartialEq)]
struct Node<T> {
    next: Link<T>,
    data: T
}

impl<T> Node<T> {
    pub fn next_as_ref(&self) -> &Link<T> {
        &self.next
    }

    pub fn next_as_mut(&mut self) -> &mut Link<T> {
        &mut self.next
    }

    pub fn get_data(&mut self) -> &T {
        &self.data
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.data)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct LinkedList<T> {
    head: Link<T>,
    length: usize
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0
        }
    }

    pub fn head_as_ref(&self) -> &Link<T> {
        &self.head
    }

    pub fn len(&self) -> usize {
        return self.length
    }

    pub fn push_front(&mut self, data: T) -> () {
        let new_node = SmartPointer::new(
            Node {
                next: self.head.take(),
                data
            }
        );
        self.head = Some(new_node);
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> where T: Copy {
        self.head.take().map(|mut node| {
            let data_copy = node.data;
            self.head = node.next.take();
            self.length -= 1;
            data_copy
        })
    }

    pub fn sum(&self) -> T
        where T: Copy + AddAssign + Default
    {
        let mut sum: T = T::default();
        let mut current = self.head_as_ref();
        while !current.is_none() {
            sum += current.as_ref().unwrap().data;
            current = current.as_ref().unwrap().next_as_ref();
        }
        sum
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {

        let mut current = self.head_as_ref();
        if current.is_none() {
            write!(f, "None")?;
        }
        while !current.is_none() {
            write!(f, "{}", current.as_ref().unwrap().data)?;
            current = current.as_ref().unwrap().next_as_ref();
            if current.is_some() {
                write!(f, " -> ")?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod linked_lists {
    use std::fmt::{Display, format};
    use crate::structs::linked_lists::{LinkedList, Node};
    use crate::structs::smart_ptr::SmartPointer;

    #[test]
    fn test_new() {
        let ll: LinkedList<u8> = LinkedList::new();
        assert_eq!(ll.head, None, "Linked List has invalid initial Head!")
    }

    #[test]
    fn test_push_front() {
        let mut ll: LinkedList<u8> = LinkedList::new();

        ll.push_front(5);
        ll.push_front(10);

        let head = ll.head_as_ref().as_ref().unwrap();
        let next = ll.head_as_ref().as_ref().unwrap().next_as_ref().as_ref().unwrap();
        assert_eq!(head.data, 10, "Linked List has invalid data at head!");
        assert_eq!(next.data, 5, "Linked List has invalid set at Node next!");
    }

    #[test]
    fn test_pop_front() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);

        assert_eq!(ll.pop_front().unwrap(), 10, "Linked List Pop operation returned invalid popped data!");
        assert_eq!(ll.head_as_ref().as_ref().unwrap().data, 5, "Linked List has invalid data at head after pop operation!");
        assert_eq!(ll.head_as_ref().as_ref().unwrap().next, None, "Linked List has invalid set at Node next after pop operation!");
        ll.pop_front();
        assert!(ll.pop_front().is_none(), "Empty Linked List returned invalid popped data. It should have been None!");
    }

    #[test]
    fn test_len() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        assert_eq!(ll.len(), 2, "Linked List invalid nodes length!");
        ll.pop_front();
        assert_eq!(ll.len(), 1, "Linked List invalid nodes length!")
    }

    #[test]
    fn test_head_as_ref() {
        let ll: LinkedList<u8> = LinkedList::new();
        assert_eq!(ll.head, None, "Linked List has invalid initial Head!")
    }

    #[test]
    fn test_sum() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        assert_eq!(ll.sum(), 15, "Invalid Linked List sum!");
    }

    #[test]
    fn test_display_trait() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        assert_eq!(format!("{}", ll), format!("10 -> 5"), "Linked List has invalid Display trait!");
    }
}