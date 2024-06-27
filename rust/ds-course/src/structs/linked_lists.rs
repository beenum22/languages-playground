use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Deref};
use num::Bounded;
use crate::structs::smart_ptrs::{SharedSmartPointer};

type Link<T> = Option<SharedSmartPointer<Node<T>>>;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
struct Node<T> {
    next: Link<T>,
    previous: Link<T>,
    data: T
}

impl<T> Node<T> {
    pub fn next_as_ref(&self) -> Option<&SharedSmartPointer<Node<T>>> {
        self.next.as_ref()
    }

    pub fn next_as_mut(&mut self) -> Option<&mut SharedSmartPointer<Node<T>>> {
        self.next.as_mut()
    }

    pub fn get_data(&mut self) -> &T {
        &self.data
    }

    pub fn set_next(&mut self, link: Link<T>) -> &Link<T> {
        self.next = link;
        &self.next
    }

    pub fn set_previous(&mut self, link: Link<T>) -> &Link<T> {
        self.previous = link;
        &self.previous
    }
}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ", self.data)?;
        Ok(())
    }
}

// impl<T: Clone> Clone for Node<T> {
//     fn clone(&self) -> Self {
//         Self {
//             next: None,
//             previous: None,
//             data: (),
//         }
//     }
// }

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

    pub fn head_as_ref(&self) -> Option<&SharedSmartPointer<Node<T>>> {
        self.head.as_ref()
    }

    pub fn head_as_mut(&mut self) -> Option<&mut SharedSmartPointer<Node<T>>> {
        self.head.as_mut()
    }

    // TODO: Finish Swapping
    pub fn swap(&mut self, left: Link<T>, right: Link<T>) -> () {
        // left.
        // let mut temp_1 = self.head_as_mut().unwrap().next;
    }

    pub fn peek(&self) -> Option<&T> {
        return self.head_as_ref().map(|node| {
            &node.data
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        return self.head_as_mut().map(|node| {
            &mut node.data
        })
    }

    pub fn len(&self) -> usize {
        return self.length
    }

    pub fn next_as_ref(&self) -> Option<&SharedSmartPointer<Node<T>>> {
        match self.head.as_ref() {
            Some(head) => head.next_as_ref(),
            None => None
        }
    }

    pub fn previous_as_ref(&self) -> Option<&SharedSmartPointer<Node<T>>> {
        match self.head.as_ref() {
            Some(head) => head.next_as_ref(),
            None => None
        }
    }

    pub fn push_front(&mut self, data: T) -> () {
        let new_node = SharedSmartPointer::new(
            Node {
                next: self.head.take(),
                previous: None,
                data
            }
        );
        self.head = Some(new_node.clone());
        match self.head.as_mut().unwrap().next_as_mut() {
            Some(head) => {
                head.set_previous(Some(new_node.clone()))
            }
             None => &None
         };
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
            sum += current.unwrap().data;
            current = current.unwrap().next_as_ref();
        }
        sum
    }

    pub fn max(&self) -> T
        where T: Copy + Ord + Bounded
    {
        let mut max: T = T::min_value();
        let mut current = self.head_as_ref();
        while !current.is_none() {
            if current.as_ref().unwrap().data > max {
                max = current.as_ref().unwrap().data
            }
            current = current.as_ref().unwrap().next_as_ref();
        }
        max
    }

    pub fn min(&self) -> T
        where T: Copy + Ord + Bounded
    {
        let mut min: T = T::max_value();
        let mut current = self.head_as_ref();
        while !current.is_none() {
            if current.as_ref().unwrap().data < min {
                min = current.as_ref().unwrap().data
            }
            current = current.as_ref().unwrap().next_as_ref();
        }
        min
    }

    // Time Complexity is O(n)
    pub fn linear_search(&self, val: T) -> Option<&SharedSmartPointer<Node<T>>>
        where T: Copy + Ord
    {
        let mut current = self.head_as_ref();
        while !current.is_none() {
            if current.as_ref().unwrap().data == val {
                return current
            }
            current = current.as_ref().unwrap().next_as_ref();
        }
        None
    }

    // Time Complexity is ..
    pub fn move_to_head_search(&self, val: T) -> Option<&SharedSmartPointer<Node<T>>>
        where T: Copy + Ord
    {
        None
    }

    pub fn insert(&mut self, val: T) -> () {}

    pub fn sort(&mut self) -> () {}

    pub fn delete(&mut self) {}
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

// impl<T> LinkedList<T> {
//     pub fn into_iter(self) -> IntoIter<T> {
//         IntoIter(self)
//     }
//
//     pub fn iter(&self) -> Iter<T> {
//         Iter {
//             next: self.head.map(|node| &node)
//         }
//     }
// }

// pub struct IntoIter<T>(LinkedList<T>);
//
// pub struct Iter<'a, T> {
//     next: Option<&'a SmartPointer<Node<T>>>
// }
//
// impl<T: Copy> Iterator for IntoIter<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.0.pop_front()
//     }
// }

// impl<T> Iterator for Iter<T> {
//     type Item = &T;
//     fn next(&mut self) -> Option<Self::Item> {
//         self.next.map(|node| {
//             self.next =
//         })
//     }
// }

#[cfg(test)]
mod node {
    use crate::structs::linked_lists::{Link, Node};
    use crate::structs::smart_ptrs::SharedSmartPointer;

    #[test]
    fn test_set_next() {
        let mut node: Node<u8> = Node {
                next: None,
                previous: None,
                data: 1u8,
        };

        let next_link: Link<u8> = Some(
            SharedSmartPointer::new(
                Node {
                    next: None,
                    previous: None,
                    data: 1u8,
                }
            )
        );
        let next_link_ptr = next_link.as_ref().unwrap().as_ptr();

        node.set_next(next_link);
        assert_eq!(node.next.is_some(), true, "Failed to set the next link for the node!");
        assert_eq!(node.next.unwrap().as_ptr(), next_link_ptr, "Invalid next link ptr found for the node!");
    }
}

#[cfg(test)]
mod linked_list {
    use std::fmt::{Display};
    use crate::structs::linked_lists::{LinkedList};

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

        let head = ll.head_as_ref().unwrap();
        let next = ll.head_as_ref().unwrap().next_as_ref().unwrap();
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
    fn test_peek() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        assert_eq!(ll.peek(), Some(&(10u8)), "Linked List invalid head value!");
    }

    #[test]
    fn test_peek_mut() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(10);
        assert_eq!(ll.peek_mut(), Some(&mut 10u8), "Linked List invalid head value!");
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
    fn test_max() {
        let mut ll: LinkedList<i8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        ll.push_front(2);
        ll.push_front(-5);
        assert_eq!(ll.max(), 10, "Invalid Linked List max value!");
    }

    #[test]
    fn test_min() {
        let mut ll: LinkedList<i8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        ll.push_front(2);
        ll.push_front(-5);
        assert_eq!(ll.min(), -5, "Invalid Linked List min value!");
    }

    #[test]
    fn test_linear_search() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        ll.push_front(2);
        assert_eq!(ll.linear_search(10), ll.head_as_ref().as_ref().unwrap().next.as_ref(), "Invalid Linked List node for searchable value!");
        assert_eq!(ll.linear_search(11), None, "Invalid Linked List node for unsearchable value!");
    }

    #[test]
    fn test_move_to_search() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        ll.push_front(2);
        assert_eq!(ll.move_to_head_search(10), ll.head_as_ref().as_ref().unwrap().next.as_ref(), "Invalid Linked List node for searchable value!");
        assert_eq!(ll.move_to_head_search(11), None, "Invalid Linked List node for unsearchable value!");
    }

    #[test]
    fn test_display_trait() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        assert_eq!(format!("{}", ll), format!("10 -> 5"), "Linked List has invalid Display trait!");
    }
}

// #[cfg(test)]
// mod iterator {
//     use crate::structs::linked_lists::LinkedList;
//
//     #[test]
//     fn test_list_into_iter() {
//         let mut list: LinkedList<u8> = LinkedList::new();
//         list.push_front(1); list.push_front(2); list.push_front(3);
//
//         let mut iter = list.into_iter();
//         assert_eq!(iter.next(), Some(3));
//         assert_eq!(iter.next(), Some(2));
//         assert_eq!(iter.next(), Some(1));
//         assert_eq!(iter.next(), None);
//     }
// }
