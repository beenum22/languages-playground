use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Deref};
use num::Bounded;
use crate::structs::smart_ptrs::{SharedSmartPointer};

type Link<T> = Option<SharedSmartPointer<Node<T>>>;

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Clone)]
pub struct Node<T> {
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

    pub fn previous_as_ref(&self) -> Option<&SharedSmartPointer<Node<T>>> {
        self.previous.as_ref()
    }

    pub fn previous_as_mut(&mut self) -> Option<&mut SharedSmartPointer<Node<T>>> {
        self.previous.as_mut()
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
    tail: Link<T>,
    length: usize
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0
        }
    }

    pub fn head_as_ref(&self) -> Option<&SharedSmartPointer<Node<T>>> {
        self.head.as_ref()
    }

    pub fn head_as_mut(&mut self) -> Option<&mut SharedSmartPointer<Node<T>>> {
        self.head.as_mut()
    }

    pub fn tail_as_ref(&self) -> Option<&SharedSmartPointer<Node<T>>> {
        self.tail.as_ref()
    }

    pub fn tail_as_mut(&mut self) -> Option<&mut SharedSmartPointer<Node<T>>> {
        self.tail.as_mut()
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

    pub fn head_next_as_ref(&self) -> Option<&SharedSmartPointer<Node<T>>> {
        match self.head.as_ref() {
            Some(head) => head.next_as_ref(),
            None => None
        }
    }

    pub fn head_previous_as_ref(&self) -> Option<&SharedSmartPointer<Node<T>>> {
        match self.head.as_ref() {
            Some(head) => head.previous_as_ref(),
            None => None
        }
    }

    // Time Complexity is O(1)
    pub fn push_front(&mut self, data: T) -> () {
        let mut new_node = SharedSmartPointer::new(
            Node {
                next: None,
                previous: None,
                data
            }
        );
        match self.head.take() {
            Some(mut node) => {
                node.set_previous(Some(new_node.clone()));
                new_node.set_next(Some(node));
            }
            None => {
                self.tail = Some(new_node.clone());
            }
        };
        self.head = Some(new_node.clone());
        self.length += 1;
    }

    // Time Complexity is O(1)
    pub fn push_back(&mut self, data: T) -> () {
        let mut new_node = SharedSmartPointer::new(
            Node {
                next: None,
                previous: None,
                data
            }
        );
        match self.tail.take() {
            Some(mut node) => {
                node.set_next(Some(new_node.clone()));
                new_node.set_previous(Some(node));
            }
            None => {
                self.head = Some(new_node.clone());
            }
        };
        self.tail = Some(new_node.clone());
        self.length += 1;
    }

    // Time Complexity is O(1)
    pub fn pop_front(&mut self) -> Option<T> where T: Copy
    {
        match self.head.take() {
            Some(mut node) => {
                let data_copy = node.data;
                self.head = node.next.take();
                if self.head_as_ref().is_some() {
                    self.head_as_mut().unwrap().set_previous(None);
                }
                self.length -= 1;
                if self.length == 0 {
                    self.tail = None;
                }
                Some(data_copy)
            },
            None => None
        }
    }

    // Time Complexity is O(1)
    pub fn pop_back(&mut self) -> Option<T> where T: Copy
    {
        match self.tail.take() {
            Some(mut node) => {
                let data_copy = node.data;
                self.tail = node.previous.take();
                if self.tail_as_ref().is_some() {
                    self.tail_as_mut().unwrap().set_next(None);
                }
                self.length -= 1;
                if self.length == 0 {
                    self.head = None;
                }
                Some(data_copy)
            },
            None => None
        }
    }

    // Time Complexity is O(n)
    pub fn insert(&mut self, index: usize, data: T) -> () {
        if index > self.length {
            panic!("Index out of bounds! We can only insert at existing indices or after the last node.");
        }
        let mut current = self.head_as_ref();
        for i in 0..self.length + 1 {
            if i == index {
                let mut new_node = SharedSmartPointer::new(
                    Node {
                        next: None,
                        previous: None,
                        data
                    }
                );
                match current {
                    Some(_) => {
                        let mut current_clone = current.unwrap().clone();
                        match i == self.length {
                            false => {
                                match current_clone.previous_as_mut() {
                                    Some(node) => {
                                        node.set_next(Some(new_node.clone()));  // Set current->prev->next to new_node
                                        new_node.set_previous(Some(node.clone()));
                                    },
                                    None => {
                                        new_node.set_previous(None);
                                        self.head = Some(new_node.clone());  // Because it will be the new head
                                    }
                                }
                                new_node.set_next(Some(current_clone.clone()));
                                current_clone.set_previous(Some(new_node));
                            },
                            true => {
                                new_node.set_previous(Some(current_clone.clone()));
                                current_clone.set_next(Some(new_node.clone()));
                                self.tail = Some(new_node);
                            }
                        }
                    },
                    None => {
                        self.head = Some(new_node.clone());
                        self.tail = Some(new_node.clone());
                    }
                }
                self.length += 1;
                break;
            }

            if i < self.length - 1 {
                current = match current {
                    Some(node) => node.next_as_ref(),
                    None => current,
                };
            }
        }
    }

    // Time Complexity is O(n)
    pub fn delete(&mut self, index: usize) -> () {
        if index >= self.length {
            panic!("Index out of bounds!");
        }
        let len = self.length;
        let mut current = self.head_as_ref();
        for i in 0..len {
            if i == index {
                match current {
                    Some(_) => {
                        let mut current_clone = current.unwrap().clone();
                        if current_clone.previous_as_ref().is_none() {
                            if current_clone.next_as_ref().is_some() {
                                current_clone.next_as_mut().unwrap().set_previous(None);
                            }
                            self.head = current_clone.next.clone()
                        }
                        if current_clone.next_as_ref().is_none() {
                            if current_clone.previous_as_ref().is_none() {
                                self.head = None;
                                self.tail = None;
                            } else {
                                current_clone.previous_as_mut().unwrap().set_next(None);
                                self.tail = current_clone.previous.clone();
                            }
                        }
                    }
                    None => ()
                }
                self.length -= 1;
                break;
            }
            current = current.unwrap().next_as_ref();
        }
    }

    // TODO: Finish Swapping
    pub fn swap(&mut self, left: usize, right: usize) -> Result<(), &'static str> {
        if left >= self.length || right >= self.length {
            return Err("Index out of bounds!")
        }
        if left == right {
            return Ok(())
        }
        // let mut current = self.head_as_ref();
        // let mut left_clone: Option<SharedSmartPointer<Node<T>>>;
        // let mut right_clone: Option<SharedSmartPointer<Node<T>>>;
        // for i in 0..self.length {
        //     if i == left {
        //         left_clone = Some(current.unwrap().clone());
        //     } else if i == right {
        //         right_clone = Some(current.unwrap().clone());
        //     }
        //
        //     if left_clone.is_some() && right_clone.is_some() {
        //
        //     }
        //
        //     current = current.unwrap().next_as_ref();
        // }
        Ok(())
    }

    // Time Complexity is O(n)
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

    // Time Complexity is O(n)
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

    // Time Complexity is O(n)
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
        where T: Ord
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

    // TODO: The method looks very messy with a lot of Clones. Maybe it's fine but check other ways.
    // Time Complexity is ..
    pub fn move_to_head_search(&mut self, val: T) -> Option<&SharedSmartPointer<Node<T>>>
        where T: Ord
    {
        let mut current = Some(self.head_as_ref().unwrap().clone());
        while !current.is_none() {
            if current.as_ref().unwrap().data == val {
                if current.as_ref() != self.head_as_ref() {
                    let current_prev = current.as_ref().unwrap().previous.clone();
                    let current_next = current.as_ref().unwrap().next.clone();
                    if current.as_mut().unwrap().next_as_mut().is_some() {
                        current.as_mut().unwrap().previous_as_mut().unwrap().set_next(current_next);  // Adjust the older previous Node's next Node
                        current.as_mut().unwrap().next_as_mut().unwrap().set_previous(current_prev);  // Adjust the older next Node's previous Node
                    } else {
                        current.as_mut().unwrap().previous_as_mut().unwrap().set_next(None);  // Adjust the older previous Node's next Node
                    }

                    current.as_mut().unwrap().set_previous(None);  // Set previous Node to None since it's the new head Node
                    current.as_mut().unwrap().set_next(Some(self.head_as_mut().unwrap().clone()));  // Set next Node to old head Node


                    self.head = Some(current.unwrap().clone());  // Update head Node to the newly found Node
                }
                return self.head_as_ref()
            }
            current = Some(current.unwrap().next_as_mut().unwrap().clone());
        }
        None
    }

    pub fn sort(&mut self) -> () {}
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

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        for _i in 0..self.length {
            match current {
                Some(mut node) => {
                    current = node.next.take();
                }
                None => return ()
            }
        }
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
        let next = ll.next_as_ref().unwrap();
        assert_eq!(ll.head_as_ref().unwrap().data, 10, "Head Node has invalid data!");
        assert_eq!(ll.previous_as_ref().is_none(), true, "Head Node has invalid previous Node set!");
        assert_eq!(ll.next_as_ref().unwrap().data, 5, "Next Node has invalid data!");
        assert_eq!(ll.next_as_ref().unwrap().previous_as_ref().unwrap().data, 10, "Next Node has invalid previous Node set!");
        assert_eq!(ll.next_as_ref().unwrap().next_as_ref().is_none(), true, "Next Node has invalid next Node set!");
    }

    #[test]
    fn test_pop_front() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);

        assert_eq!(ll.pop_front().unwrap(), 10, "Pop operation returned invalid popped data!");
        assert_eq!(ll.head_as_ref().unwrap().data, 5, "Head Node has invalid data after pop!");
        assert_eq!(ll.next_as_ref().is_none(), true, "Head Node has invalid next Node set!");
        assert_eq!(ll.previous_as_ref().is_none(), true, "Head Node has invalid previous Node set!");
        ll.pop_front();
        assert!(ll.pop_front().is_none(), "Empty List returned invalid popped data. It should have been None!");
    }

    #[test]
    fn test_insert() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        // ll.push_front(5);

        ll.insert(0, 5);
        assert_eq!(format!("{}", ll), "5".to_string(), "Linked List is invalid after insert in empty list!");
        assert_eq!(ll.length, 1, "Linked List has invalid length after insert!");

        ll.insert(0, 7);
        assert_eq!(format!("{}", ll), "7 -> 5".to_string(), "Linked List is invalid after insert at the start!");
        assert_eq!(ll.length, 2, "Linked List has invalid length after insert!");
        assert_eq!(ll.next_as_ref().unwrap().previous_as_ref().is_some(), true, "Moved Node has invalid previous Node set!");
        assert_eq!(ll.next_as_ref().unwrap().next_as_ref().is_none(), true, "Moved Node has invalid next Node set!");

        ll.insert(1, 6);
        assert_eq!(format!("{}", ll), "7 -> 6 -> 5".to_string(), "Linked List is invalid after insert in the middle!");
        assert_eq!(ll.length, 3, "Linked List has invalid length after insert!");
        assert_eq!(ll.next_as_ref().unwrap().previous_as_ref().is_some(), true, "Moved Node has invalid previous Node set!");
        assert_eq!(ll.next_as_ref().unwrap().next_as_ref().is_some(), true, "Moved Node has invalid next Node set!");

        ll.insert(3, 4);
        assert_eq!(format!("{}", ll), "7 -> 6 -> 5 -> 4".to_string(), "Linked List is invalid after insert next to last node!");
        assert_eq!(ll.length, 4, "Linked List has invalid length after insert!");
        assert_eq!(ll.linear_search(4).unwrap().next_as_ref().is_none(), true, "New last Node has invalid next Node set!");
        assert_eq!(ll.linear_search(4).unwrap().previous_as_ref().is_some(), true, "New last Node has invalid previous Node set!");

        assert_eq!(
            ll.insert(5, 10).unwrap_err(),
            "Index out of bounds! We can only insert at existing indices or after the last node.",
            "Linked List invalid index exception failed!"
        );
    }

    #[test]
    fn test_delete() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        ll.push_front(15);
        ll.push_front(20);
        ll.push_front(25);

        ll.delete(0);
        assert_eq!(format!("{}", ll), "20 -> 15 -> 10 -> 5".to_string(), "Linked List is invalid after first index deletion!");
        assert_eq!(ll.length, 4, "Linked List has invalid length after delete!");

        ll.delete(3);
        assert_eq!(format!("{}", ll), "20 -> 15 -> 10".to_string(), "Linked List is invalid after last index deletion!");
        assert_eq!(ll.length, 3, "Linked List has invalid length after delete!");

        ll.delete(2);
        assert_eq!(format!("{}", ll), "20 -> 15".to_string(), "Linked List is invalid after middle index deletion!");
        assert_eq!(ll.length, 2, "Linked List has invalid length after delete!");

        ll.delete(2);
        assert_eq!(
            ll.delete(5).unwrap_err(),
            "Index out of bounds!",
            "Linked List didn't throw exception after invalid index deletion!"
        );

        ll.delete(1);
        ll.delete(0);
        assert_eq!(format!("{}", ll), "None".to_string(), "Linked List is invalid after all indices deletion!");
        assert_eq!(ll.length, 0, "Linked List has invalid length after delete!");
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
    fn test_move_to_head_search() {
        let mut ll: LinkedList<u8> = LinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        ll.push_front(2);

        let node_1 = ll.linear_search(5).unwrap().clone();
        let node_2 = ll.linear_search(10).unwrap().clone();
        let node_3 = ll.linear_search(2).unwrap().clone();
        // println!("Node 1 (5) = {:?}", node_1);
        // println!("Node 2 (10) = {:?}", node_2);
        // println!("Node 3 (2) = {:?}", node_3);

        // Original List: 2 -> 10 -> 5
        // Moved List: 10 -> 2 -> 5

        // Try moving some in between Node
        assert_eq!(*ll.move_to_head_search(10).unwrap(), node_2, "Failed to return the found Node!");
        assert_eq!(ll.previous_as_ref().is_none(), true, "Moved Node has invalid previous Node set!");
        assert_eq!(*ll.next_as_ref().unwrap(), node_3, "Moved Node has invalid next Node set!");
        assert_eq!(*node_3.next_as_ref().unwrap(), node_1, "Old previous Node has invalid adjusted next Node set!");
        assert_eq!(*node_1.previous_as_ref().unwrap(), node_3, "Old next Node has invalid adjusted previous Node set!");

        // Try moving the already moved Node or head Node.
        assert_eq!(*ll.move_to_head_search(10).unwrap(), node_2, "Failed to return the found Node!");
        assert_eq!(ll.previous_as_ref().is_none(), true, "Unmoved Node has invalid previous Node set!");
        assert_eq!(*ll.next_as_ref().unwrap(), node_3, "Unmoved Node has invalid next Node set!");
        println!("{}", ll);
        // Try moving the last Node
        println!("F Node = {:?}", node_2.next_as_ref().unwrap());
        assert_eq!(*ll.move_to_head_search(5).unwrap(), node_1, "Failed to return the found last Node!");
        assert_eq!(node_3.next_as_ref().is_none(), true, "Last moved Node has invalid next Node set!");
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
