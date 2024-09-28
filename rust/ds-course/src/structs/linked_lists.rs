/*
NOTE:
This struct implements a Doubly Linked List that requires the usage of Reference Counter and Shared owned resources that impacts the performance.
It won't be possible implement a Doubly Linked List without such limitations. However, we can implement a Singly Linked List without the use of Reference Counter or bypassing the ownership rules.
*/
use crate::structs::arrays::HeapArray;
use crate::structs::smart_ptrs::{AtomicReferenceCounter, HeapBox};
use num::Bounded;
use std::fmt::{Debug, Display, format, Formatter};
use std::ops::{AddAssign, Deref};
use std::ptr;

pub trait LinkedListADT<T> {
    fn head_as_ref(&self) -> Option<&NodeType<T>>;

    fn head_as_mut(&mut self) -> Option<&mut NodeType<T>>;

    fn tail_as_ref(&self) -> Option<&NodeType<T>> {
        None
    }

    fn tail_as_mut(&mut self) -> Option<&mut NodeType<T>> {
        None
    }

    fn push_front(&mut self, data: T) -> ();

    fn push_back(&mut self, data: T) -> ();

    fn pop_front(&mut self) -> Option<T>
    where
        T: Copy;

    fn pop_back(&mut self) -> Option<T>
    where
        T: Copy;

    fn peek(&self, index: usize) -> &T;

    fn peek_mut(&mut self, index: usize) -> &mut T;

    fn len(&self) -> usize;
}

#[derive(Debug, PartialEq)]
pub enum NodeType<T> {
    Singly(HeapBox<Node<T>>),
    Doubly(AtomicReferenceCounter<Node<T>>),
}

impl<T: Clone> Clone for NodeType<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Singly(node) => Self::Singly(node.clone()),
            Self::Doubly(node) => Self::Doubly(node.clone()),
        }
    }
}

impl<T> AsRef<Node<T>> for NodeType<T> {
    fn as_ref(&self) -> &Node<T> {
        match self {
            Self::Singly(node) => node,
            Self::Doubly(node) => node,
        }
    }
}

impl<T> NodeType<T> {
    fn new_singly(val: T) -> Self {
        Self::Singly(HeapBox::new(Node {
            next: None,
            previous: None,
            data: val,
        }))
    }

    fn new_doubly(val: T) -> Self {
        Self::Doubly(AtomicReferenceCounter::new(Node {
            next: None,
            previous: None,
            data: val,
        }))
    }

    fn as_ptr(&self) -> *const Node<T> {
        match self {
            NodeType::Singly(node) => node.as_ptr() as *const Node<T>,
            NodeType::Doubly(node) => node.as_val_ptr(),
        }
    }

    fn next_as_ref(&self) -> Option<&NodeType<T>> {
        match self {
            NodeType::Singly(node) => node.next.as_ref(),
            NodeType::Doubly(node) => node.next.as_ref(),
        }
    }

    fn next_as_mut(&mut self) -> Option<&mut NodeType<T>> {
        match self {
            NodeType::Singly(node) => node.next.as_mut(),
            NodeType::Doubly(node) => node.next.as_mut(),
        }
    }

    fn previous_as_ref(&self) -> Option<&NodeType<T>> {
        match self {
            NodeType::Singly(node) => node.previous.as_ref(),
            NodeType::Doubly(node) => node.previous.as_ref(),
        }
    }

    fn previous_as_mut(&mut self) -> Option<&mut NodeType<T>> {
        match self {
            NodeType::Singly(node) => node.previous.as_mut(),
            NodeType::Doubly(node) => node.previous.as_mut(),
        }
    }

    fn next(&mut self) -> Option<NodeType<T>> {
        match self {
            NodeType::Singly(node) => node.next.take(),
            NodeType::Doubly(node) => node.next.take(),
        }
    }

    fn previous(&mut self) -> Option<NodeType<T>> {
        match self {
            NodeType::Singly(node) => node.previous.take(),
            NodeType::Doubly(node) => node.previous.take(),
        }
    }

    pub fn data_as_ref(&self) -> &T {
        match self {
            NodeType::Singly(node) => &node.data,
            NodeType::Doubly(node) => &node.data,
        }
    }

    fn data_as_mut(&mut self) -> &mut T {
        match self {
            NodeType::Singly(node) => &mut node.data,
            NodeType::Doubly(node) => &mut node.data,
        }
    }

    fn data(&mut self) -> T
    where
        T: Copy,
    {
        match self {
            NodeType::Singly(node) => node.data,
            NodeType::Doubly(node) => node.data,
        }
    }

    fn set_next(&mut self, next: Option<NodeType<T>>) -> Option<&NodeType<T>> {
        match self {
            NodeType::Singly(node) => {
                node.next = next;
                node.next.as_ref()
            }
            NodeType::Doubly(node) => {
                node.next = next;
                node.next.as_ref()
            }
        }
    }

    fn set_previous(&mut self, previous: Option<NodeType<T>>) -> Option<&NodeType<T>> {
        match self {
            NodeType::Singly(node) => {
                node.previous = previous;
                node.previous.as_ref()
            }
            NodeType::Doubly(node) => {
                node.previous = previous;
                node.previous.as_ref()
            }
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct Node<T> {
    next: Option<NodeType<T>>,
    previous: Option<NodeType<T>>,
    data: T,
}

impl<T> AsMut<T> for Node<T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.data
    }
}

impl<T> AsRef<T> for Node<T> {
    fn as_ref(&self) -> &T {
        &self.data
    }
}

impl<T: Debug> Debug for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Node")
            .field("data", &self.data)
            .field("next", &match &self.next {
                Some(node) => node.as_ptr(),
                _ => ptr::null()
            })
            .field("previous", &match &self.previous {
                Some(node) => node.as_ptr(),
                _ => ptr::null()
            })
            .finish()
    }

}

impl<T: Display> Display for Node<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.data)?;
        Ok(())
    }
}

struct LinkedListIterator<'a, T> {
    current: Option<&'a NodeType<T>>,
}

impl<'a, T> Iterator for LinkedListIterator<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next_as_ref();
            node.data_as_ref()
        })
    }
}

pub struct SinglyLinkedList<T> {
    head: Option<NodeType<T>>,
    length: usize,
}

impl<T> LinkedListADT<T> for SinglyLinkedList<T> {
    fn head_as_ref(&self) -> Option<&NodeType<T>> {
        self.head.as_ref()
    }

    fn head_as_mut(&mut self) -> Option<&mut NodeType<T>> {
        self.head.as_mut()
    }

    // Time Complexity is O(1)
    fn push_front(&mut self, data: T) -> () {
        let new_node = NodeType::Singly(HeapBox::new(Node {
            next: self.head.take(),
            previous: None,
            data,
        }));
        self.head = Some(new_node);
        self.length += 1;
    }

    // Time Complexity is O(n)
    fn push_back(&mut self, data: T) -> () {
        if self.head.is_none() {
            self.head = Some(NodeType::Singly(HeapBox::new(Node {
                next: None,
                previous: None,
                data,
            })));
            return;
        }
        match self.head.is_some() {
            true => {
                let mut current = self.head_as_mut();
                while current.as_ref().unwrap().next_as_ref().is_some() {
                    current = current.unwrap().next_as_mut()
                }
                current.unwrap().set_next(Some(NodeType::new_singly(data)));
            }
            false => {
                self.head = Some(NodeType::Singly(HeapBox::new(Node {
                    next: None,
                    previous: None,
                    data,
                })))
            }
        }
        self.length += 1;
    }

    // Time Complexity is O(n)
    fn pop_front(&mut self) -> Option<T>
    where
        T: Copy,
    {
        match self.head.take() {
            Some(mut head) => {
                self.head = head.next();
                self.length -= 1;
                Some(head.data())
            }
            None => None,
        }
    }

    // Time Complexity is O(n)
    fn pop_back(&mut self) -> Option<T>
    where
        T: Copy,
    {
        if self.head.is_none() {
            return None;
        }

        if self.head_as_ref().unwrap().next_as_ref().is_none() {
            self.length -= 1;
            return self.head.take().map(|mut node| node.data());
        }

        let mut len: usize = self.length;
        let mut current = self.head_as_mut();
        for i in 0..len - 2 {
            current = current.unwrap().next_as_mut()
        }
        let next_node = current.as_mut().unwrap().next();
        current.unwrap().set_next(None);
        self.length -= 1;
        Some(next_node.unwrap().data())
    }

    fn peek(&self, index: usize) -> &T {
        if index >= self.length {
            panic!("Index out of bounds!");
        }
        let mut current = self.head_as_ref();
        let mut i: usize = 0;
        while i != index {
            current = current.unwrap().next_as_ref();
            i += 1;
        }
        current.unwrap().data_as_ref()
    }

    fn peek_mut(&mut self, index: usize) -> &mut T {
        if index >= self.length {
            panic!("Index out of bounds!");
        }
        let mut current = self.head_as_mut();
        let mut i: usize = 0;
        while i != index {
            current = current.unwrap().next_as_mut();
            i += 1;
        }
        current.unwrap().data_as_mut()
    }

    fn len(&self) -> usize {
        self.length
    }
}

impl<T> SinglyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            length: 0,
        }
    }

    pub fn iter(&self) -> LinkedListIterator<T> {
        LinkedListIterator {
            current: self.head_as_ref(),
        }
    }
}

impl<T: Display> Display for SinglyLinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head_as_ref();
        if current.is_none() {
            write!(f, "None")?;
        }
        while !current.is_none() {
            write!(f, "{}", current.as_ref().unwrap().data_as_ref())?;
            current = current.as_ref().unwrap().next_as_ref();
            if current.is_some() {
                write!(f, " -> ")?;
            }
        }
        Ok(())
    }
}

pub struct DoublyLinkedList<T> {
    pub(crate) head: Option<NodeType<T>>,
    tail: Option<NodeType<T>>,
    length: usize,
}

impl<T> DoublyLinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }
}

impl<T> LinkedListADT<T> for DoublyLinkedList<T>
where
    T: Clone,
{
    fn head_as_ref(&self) -> Option<&NodeType<T>> {
        self.head.as_ref()
    }

    fn head_as_mut(&mut self) -> Option<&mut NodeType<T>> {
        self.head.as_mut()
    }

    fn tail_as_ref(&self) -> Option<&NodeType<T>> {
        self.tail.as_ref()
    }

    fn tail_as_mut(&mut self) -> Option<&mut NodeType<T>> {
        self.tail.as_mut()
    }

    fn push_front(&mut self, data: T) -> () {
        let mut new_node = NodeType::new_doubly(data);

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

    fn push_back(&mut self, data: T) -> () {
        let mut new_node = NodeType::new_doubly(data);
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

    fn pop_front(&mut self) -> Option<T>
    where
        T: Copy,
    {
        match self.head.take() {
            Some(mut node) => {
                let data_copy = node.data();
                self.head = node.next().take();
                if self.head_as_ref().is_some() {
                    self.head_as_mut().unwrap().set_previous(None);
                }
                self.length -= 1;
                if self.length == 0 {
                    self.tail = None;
                }
                Some(data_copy)
            }
            None => None,
        }
    }

    fn pop_back(&mut self) -> Option<T>
    where
        T: Copy,
    {
        match self.tail.take() {
            Some(mut node) => {
                let data_copy = node.data();
                self.tail = node.previous().take();
                if self.tail_as_ref().is_some() {
                    self.tail_as_mut().unwrap().set_next(None);
                }
                self.length -= 1;
                if self.length == 0 {
                    self.head = None;
                }
                Some(data_copy)
            }
            None => None,
        }
    }

    fn peek(&self, index: usize) -> &T {
        if index >= self.length {
            panic!("Index out of bounds!");
        }
        let len = self.length;
        let mut current = self.head_as_ref();
        let mut i: usize = 0;
        while i != index {
            current = current.unwrap().next_as_ref();
            i += 1;
        }
        current.unwrap().data_as_ref()
    }

    fn peek_mut(&mut self, index: usize) -> &mut T {
        if index >= self.length {
            panic!("Index out of bounds!");
        }
        let len = self.length;
        let mut current = self.head_as_mut();
        let mut i: usize = 0;
        while i != index {
            current = current.unwrap().next_as_mut();
            i += 1;
        }
        current.unwrap().data_as_mut()
    }

    fn len(&self) -> usize {
        return self.length;
    }
}

impl<T> DoublyLinkedList<T> {
    // Time Complexity is O(n)
    pub fn insert(&mut self, index: usize, data: T) -> ()
    where
        T: Clone,
    {
        if index >= self.length {
            panic!("Index out of bounds!");
        }
        let mut new_node = NodeType::new_doubly(data);
        // TODO: AtomicReferenceCounter might need weak reference implementation. Check if we can avoid clones here.
        let mut current = self.head.as_ref().unwrap().clone();
        let mut i: usize = 0;
        while i != index {
            current = current.next().unwrap().clone();
            i += 1
        }
        match current.previous_as_mut() {
            Some(prev_node) => {
                prev_node.set_next(Some(new_node.clone()));
                new_node.set_previous(Some(prev_node.clone()));
            }
            None => {
                new_node.set_previous(None);
                self.head = Some(new_node.clone());
            }
        }
        new_node.set_next(Some(current.clone()));
        current.set_previous(Some(new_node.clone()));
        self.length += 1;
    }

    // Time Complexity is O(n)
    pub fn delete(&mut self, index: usize) -> ()
    where
        T: Clone + Debug
    {
        if index >= self.length {
            panic!("Index out of bounds!");
        }

        let mut head_clone = self.head.clone();
        let mut current = head_clone.as_mut().unwrap();
        let mut i: usize = 0;

        while i != index {
            current = current.next_as_mut().unwrap();
            i += 1
        }

        let mut next_node = current.clone().next().take();
        let mut previous_node = current.clone().previous().take();

        if current.previous_as_ref().is_some() {
            current.previous_as_mut().unwrap().set_next(next_node.clone());
        }
        if current.next_as_ref().is_some() {
            current.next_as_mut().unwrap().set_previous(previous_node.clone());
        }

        if previous_node.is_none() {
            self.head = next_node.clone();
            self.head_as_mut().unwrap().set_previous(None);
        }

        if next_node.is_none() {
            self.tail = previous_node.clone();
            self.tail_as_mut().unwrap().set_next(None);
        }
        self.length -= 1;
    }
}

impl<T: Display + Clone> Display for DoublyLinkedList<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut current = self.head_as_ref();
        if current.is_none() {
            write!(f, "None")?;
        }
        while !current.is_none() {
            write!(f, "{}", current.as_ref().unwrap().data_as_ref())?;
            current = current.as_ref().unwrap().next_as_ref();
            if current.is_some() {
                write!(f, " -> ")?;
            }
        }
        Ok(())
    }
}

impl<T> Drop for DoublyLinkedList<T> {
    fn drop(&mut self) {
        let mut current = self.head.take();
        for _i in 0..self.length {
            match current {
                Some(mut node) => {
                    current = node.next().take();
                }
                None => return (),
            }
        }
    }
}

#[cfg(test)]
mod node {
    use crate::structs::linked_lists::{NodeType, Node};

    #[test]
    fn test_next_as_ref() {
        let mut node = NodeType::new_singly(1u8);
        assert_eq!(
            node.next_as_ref().is_none(),
            true,
            "Next node must be None!"
        );

        node.set_next(Some(NodeType::new_singly(2u8)));
        assert_eq!(
            node.next_as_ref().is_some(),
            true,
            "Next node must NOT be None!"
        );
    }

    #[test]
    fn test_next_as_mut() {
        let mut node = NodeType::new_singly(1u8);

        assert_eq!(
            node.next_as_mut().is_none(),
            true,
            "Next node must be None!"
        );

        node.set_next(Some(NodeType::new_singly(2u8)));
        assert_eq!(
            node.next_as_mut().is_some(),
            true,
            "Next node must NOT be None!"
        );
        assert_eq!(
            node.next_as_ref().unwrap().data_as_ref(),
            &2u8,
            "Next node has invalid value!"
        );
        let mut val = node.next_as_mut().unwrap().data_as_mut();
        *val = 3u8;
        assert_eq!(
            node.next_as_ref().unwrap().data_as_ref(),
            &3u8,
            "Next node has invalid value!"
        );
    }

    #[test]
    fn test_as_ref() {
        let node: Node<u8> = Node {
            next: None,
            previous: None,
            data: 1u8,
        };

        assert_eq!(node.as_ref(), &1u8, "Node has invalid value!");
    }

    #[test]
    fn test_as_mut() {
        let mut node: Node<u8> = Node {
            next: None,
            previous: None,
            data: 1u8,
        };

        assert_eq!(node.as_mut(), &mut 1u8, "Node has invalid value!");
    }

    #[test]
    fn test_set_next() {
        let mut node = NodeType::new_singly(1u8);

        node.set_next(Some(NodeType::new_singly(2u8)));
        assert_eq!(
            node.next_as_ref().is_some(),
            true,
            "Failed to set the next link for the node!"
        );
        assert_eq!(
            node.next_as_ref().unwrap().data_as_ref(),
            &2u8,
            "Invalid next link value found for the node!"
        );
    }

    #[test]
    fn test_display_trait() {
        let node: Node<u8> = Node {
            next: None,
            previous: None,
            data: 1u8,
        };

        assert_eq!(format!("{node}"), "1", "Node has invalid value!");
    }
}

#[cfg(test)]
mod linked_list_iterator {
    use crate::structs::linked_lists::{LinkedListIterator, NodeType, Node};
    use crate::structs::smart_ptrs::HeapBox;

    #[test]
    fn test_next() {
        let mut node = NodeType::new_singly(1u8);
        let mut iter = LinkedListIterator {
            current: Some(&node),
        };
        assert_eq!(
            iter.next(),
            Some(&1u8),
            "Singly Linked List Iterator returned invalid value!!"
        );
        assert_eq!(
            iter.next(),
            None,
            "Singly Linked List Iterator returned invalid value!!"
        );
    }
}

#[cfg(test)]
mod singly_linked_list {
    use crate::structs::linked_lists::{LinkedListADT, SinglyLinkedList};

    #[test]
    fn test_new() {
        let ll: SinglyLinkedList<u8> = SinglyLinkedList::new();
        assert_eq!(
            ll.head, None,
            "Singly Linked List has invalid initial Head!"
        )
    }

    #[test]
    fn test_head_as_ref() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();
        assert_eq!(
            ll.head_as_ref(),
            None,
            "Singly Linked List has invalid initial Head reference!"
        );
        ll.push_front(1);
        assert_eq!(
            ll.head_as_ref().is_some(),
            true,
            "Linked List has None Head after push!"
        )
    }

    #[test]
    fn test_head_as_mut() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();
        assert_eq!(
            ll.head_as_mut(),
            None,
            "Singly Linked List has invalid initial Head mutable reference!"
        );
        ll.push_front(1);
        assert_eq!(
            ll.head_as_ref().is_some(),
            true,
            "Linked List has None Head after push!"
        )
    }

    #[test]
    fn test_push_back() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();

        ll.push_back(1u8);
        assert_eq!(
            ll.head.is_some(),
            true,
            "Singly Linked List Head must not be None!"
        );
        assert_eq!(
            ll.head.as_ref().unwrap().data_as_ref(),
            &1u8,
            "Singly Linked List Head has invalid value!"
        );

        ll.push_back(2u8);
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().is_some(),
            true,
            "Singly Linked List second Node must not be None!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .data_as_ref(),
            &2u8,
            "Singly Linked List second Node has invalid value!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .next_as_ref()
                .is_none(),
            true,
            "Singly Linked List second Node next must be None!"
        );
    }

    #[test]
    fn test_push_front() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();

        ll.push_front(1u8);
        assert_eq!(
            ll.head.is_some(),
            true,
            "Singly Linked List Head must not be None!"
        );
        assert_eq!(
            ll.head.as_ref().unwrap().data_as_ref(),
            &1u8,
            "Singly Linked List Head has invalid value!"
        );

        ll.push_front(2u8);
        assert_eq!(
            ll.head.as_ref().unwrap().data_as_ref(),
            &2u8,
            "Singly Linked List first Node has invalid value!"
        );
        assert_eq!(
            ll.head.as_ref().unwrap().next_as_ref().is_some(),
            true,
            "Singly Linked List second Node must not be None!"
        );
        assert_eq!(
            ll.head
                .as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .data_as_ref(),
            &1u8,
            "Singly Linked List second Node has invalid value!"
        );
        assert_eq!(
            ll.head
                .as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .next_as_ref()
                .is_none(),
            true,
            "Singly Linked List second Node next must be None!"
        );
    }

    #[test]
    fn test_pop_front() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();
        assert_eq!(
            ll.pop_front().is_none(),
            true,
            "Empty Singly Linked List pop should return None!"
        );

        ll.push_front(1u8);
        ll.push_front(2u8);
        ll.push_front(3u8);

        assert_eq!(
            ll.pop_front().unwrap(),
            3u8,
            "Singly Linked List pop returns invalid value!"
        );
        assert_eq!(
            ll.length, 2,
            "Singly Linked List has invalid length after pop!"
        );
        assert_eq!(
            ll.pop_front().unwrap(),
            2u8,
            "Singly Linked List pop returns invalid value!"
        );
        assert_eq!(
            ll.length, 1,
            "Singly Linked List has invalid length after pop!"
        );
        assert_eq!(
            ll.pop_front().unwrap(),
            1u8,
            "Singly Linked List pop returns invalid value!"
        );
        assert_eq!(
            ll.length, 0,
            "Singly Linked List has invalid length after pop!"
        );
    }

    #[test]
    fn test_pop_back() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();
        assert_eq!(
            ll.pop_back().is_none(),
            true,
            "Empty Singly Linked List pop should return None!"
        );

        ll.push_front(1u8);
        ll.push_front(2u8);
        ll.push_front(3u8);

        assert_eq!(
            ll.pop_back().unwrap(),
            1u8,
            "Singly Linked List pop returns invalid value!"
        );
        assert_eq!(
            ll.length, 2,
            "Singly Linked List has invalid length after pop!"
        );
        assert_eq!(
            ll.pop_back().unwrap(),
            2u8,
            "Singly Linked List pop returns invalid value!"
        );
        assert_eq!(
            ll.length, 1,
            "Singly Linked List has invalid length after pop!"
        );
        assert_eq!(
            ll.pop_back().unwrap(),
            3u8,
            "Singly Linked List pop returns invalid value!"
        );
        assert_eq!(
            ll.length, 0,
            "Singly Linked List has invalid length after pop!"
        );
    }

    #[test]
    #[should_panic(expected = "Index out of bounds!")]
    fn test_peak_panic() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();
        ll.peek(0);
    }

    #[test]
    fn test_peek() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();
        ll.push_front(1u8);
        ll.push_front(2u8);
        assert_eq!(ll.peek(0), &2u8, "Singly Linked List has invalid value!");
        assert_eq!(ll.peek(1), &1u8, "Singly Linked List has invalid value!");
    }

    #[test]
    #[should_panic(expected = "Index out of bounds!")]
    fn test_peak_mut_panic() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();
        ll.peek_mut(0);
    }

    #[test]
    fn test_peek_mut() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();
        ll.push_front(1u8);
        ll.push_front(2u8);
        assert_eq!(
            ll.peek_mut(0),
            &mut 2u8,
            "Singly Linked List has invalid value!"
        );
        *ll.peek_mut(0) = 3;
        assert_eq!(
            ll.peek_mut(0),
            &mut 3u8,
            "Singly Linked List has invalid value!"
        );
        assert_eq!(
            ll.peek_mut(1),
            &mut 1u8,
            "Singly Linked List has invalid value!"
        );
    }

    #[test]
    fn test_len() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();
        ll.push_front(1u8);
        ll.push_front(2u8);
        assert_eq!(ll.len(), 2, "Singly Linked List has invalid length!");
        ll.pop_front();
        assert_eq!(ll.len(), 1, "Singly Linked List has invalid length!")
    }

    #[test]
    fn test_iter() {
        let mut ll: SinglyLinkedList<u8> = SinglyLinkedList::new();
        ll.push_front(1u8);
        ll.push_front(2u8);
        let mut iter = ll.iter();
        assert_eq!(
            iter.next(),
            Some(&2u8),
            "Singly Linked List Iterator returned invalid value!!"
        );
        assert_eq!(
            iter.next(),
            Some(&1u8),
            "Singly Linked List Iterator returned invalid value!!"
        );
        assert_eq!(
            iter.next(),
            None,
            "Singly Linked List Iterator returned invalid value!!"
        );
    }
}

#[cfg(test)]
mod doubly_linked_list {
    use crate::structs::arrays::HeapArray;
    use crate::structs::linked_lists::{DoublyLinkedList, LinkedListADT, NodeType};

    #[test]
    fn test_new() {
        let ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        assert_eq!(
            ll.head, None,
            "Doubly Linked List has invalid initial Head!"
        )
    }

    #[test]
    fn test_head_as_ref() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        assert_eq!(
            ll.head_as_ref(),
            None,
            "Linked List has invalid initial Head!"
        );
        ll.push_front(1);
        assert_eq!(
            ll.head_as_ref().is_some(),
            true,
            "Linked List has None Head after push!"
        )
    }

    #[test]
    fn test_tail_as_ref() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        assert_eq!(
            ll.tail_as_ref(),
            None,
            "Linked List has invalid initial Tail!"
        );
        ll.push_front(1);
        assert_eq!(
            ll.tail_as_ref().is_some(),
            true,
            "Linked List has None Tail after push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().data_as_ref(),
            &1,
            "Linked List has invalid Tail value after push!"
        );
        ll.push_front(2);
        assert_eq!(
            ll.tail_as_ref().unwrap().data_as_ref(),
            &1,
            "Linked List has invalid Tail value after push!"
        );
    }

    #[test]
    fn test_push_front() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();

        ll.push_front(5);
        // assert_eq!(
        //     format!("{}", ll),
        //     "5".to_string(),
        //     "Doubly Linked List is invalid after first front push!"
        // );
        assert_eq!(
            ll.length, 1,
            "Doubly Linked List has invalid length after front push!"
        );
        assert_eq!(
            ll.head_as_ref().is_some(),
            true,
            "Doubly Linked List has invalid None Head after first front push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().data_as_ref(),
            &5,
            "Doubly Linked List has invalid Head after first front push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().previous_as_ref().is_none(),
            true,
            "Head has invalid previous Node after first front push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().is_none(),
            true,
            "Head has invalid next Node after first front push!"
        );
        assert_eq!(
            ll.tail_as_ref().is_some(),
            true,
            "Doubly Linked List has invalid None Tail after first front push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().data_as_ref(),
            &5,
            "Doubly Linked List has invalid Tail after first front push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().previous_as_ref().is_none(),
            true,
            "Tail has invalid previous Node after first front push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().next_as_ref().is_none(),
            true,
            "Tail has invalid next Node after first front push!"
        );

        ll.push_front(10);
        // assert_eq!(
        //     format!("{}", ll),
        //     "10 -> 5".to_string(),
        //     "Doubly Linked List is invalid after second front push!"
        // );
        assert_eq!(
            ll.length, 2,
            "Doubly Linked List has invalid length after push!"
        );
        assert_eq!(
            ll.head_as_ref().is_some(),
            true,
            "Doubly Linked List has invalid None Head after second front push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().data_as_ref(),
            &10,
            "Doubly Linked List has invalid Head after second front push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().previous_as_ref().is_none(),
            true,
            "Head has invalid previous Node after second front push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().is_some(),
            true,
            "Head has invalid None next Node after second front push!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .data_as_ref(),
            &5,
            "Head has invalid next Node after second front push!"
        );
        assert_eq!(
            ll.tail_as_ref().is_some(),
            true,
            "Doubly Linked List has invalid None Tail after second front push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().data_as_ref(),
            &5,
            "Doubly Linked List has invalid Tail after second front push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().previous_as_ref().is_some(),
            true,
            "Tail has invalid None previous Node after second front push!"
        );
        assert_eq!(
            ll.tail_as_ref()
                .unwrap()
                .previous_as_ref()
                .unwrap()
                .data_as_ref(),
            &10,
            "Tail has invalid previous Node after second front push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().next_as_ref().is_none(),
            true,
            "Tail has invalid next Node after second front push!"
        );
    }

    #[test]
    fn test_push_back() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();

        ll.push_back(5);
        // assert_eq!(
        //     format!("{}", ll),
        //     "5".to_string(),
        //     "Linked List is invalid after first back push!"
        // );
        assert_eq!(
            ll.length, 1,
            "Doubly Linked List has invalid length after back push!"
        );
        assert_eq!(
            ll.head_as_ref().is_some(),
            true,
            "Doubly Linked List has invalid None Head after first back push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().data_as_ref(),
            &5,
            "Doubly Linked List has invalid Head after first back push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().previous_as_ref().is_none(),
            true,
            "Head has invalid previous Node after first back push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().is_none(),
            true,
            "Head has invalid next Node after first back push!"
        );
        assert_eq!(
            ll.tail_as_ref().is_some(),
            true,
            "Doubly Linked List has invalid None Tail after first back push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().data_as_ref(),
            &5,
            "Doubly Linked List has invalid Tail after first back push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().previous_as_ref().is_none(),
            true,
            "Tail has invalid previous Node after first back push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().next_as_ref().is_none(),
            true,
            "Tail has invalid next Node after first back push!"
        );

        ll.push_back(10);
        // assert_eq!(
        //     format!("{}", ll),
        //     "5 -> 10".to_string(),
        //     "Doubly Linked List is invalid after second back push!"
        // );
        assert_eq!(
            ll.length, 2,
            "Doubly Linked List has invalid length after push!"
        );
        assert_eq!(
            ll.head_as_ref().is_some(),
            true,
            "Doubly Linked List has invalid None Head after second back push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().data_as_ref(),
            &5,
            "Doubly Linked List has invalid Head after second back push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().previous_as_ref().is_none(),
            true,
            "Head has invalid previous Node after second back push!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().is_some(),
            true,
            "Head has invalid None next Node after second back push!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .data_as_ref(),
            &10,
            "Head has invalid next Node after second back push!"
        );
        assert_eq!(
            ll.tail_as_ref().is_some(),
            true,
            "Doubly Linked List has invalid None Tail after second back push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().data_as_ref(),
            &10,
            "Doubly Linked List has invalid Tail after second back push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().previous_as_ref().is_some(),
            true,
            "Tail has invalid None previous Node after second back push!"
        );
        assert_eq!(
            ll.tail_as_ref()
                .unwrap()
                .previous_as_ref()
                .unwrap()
                .data_as_ref(),
            &5,
            "Tail has invalid previous Node after second back push!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().next_as_ref().is_none(),
            true,
            "Tail has invalid next Node after second back push!"
        );
    }

    #[test]
    fn test_pop_front() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        ll.push_front(15);

        ll.pop_front();
        // assert_eq!(
        //     format!("{}", ll),
        //     "10 -> 5".to_string(),
        //     "Doubly Linked List is invalid after first front pop!"
        // );
        assert_eq!(
            ll.length, 2,
            "Doubly Linked List has invalid length after pop!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().data_as_ref(),
            &10,
            "Doubly Linked List has invalid Head after first front pop!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().previous_as_ref().is_none(),
            true,
            "Head has invalid previous Node after first front pop!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().is_some(),
            true,
            "Head has invalid None next Node after first front pop!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .data_as_ref(),
            &5,
            "Head has invalid next Node after first front pop!"
        );

        assert_eq!(
            ll.tail_as_ref().unwrap().data_as_ref(),
            &5,
            "Doubly Linked List has invalid Tail after front pop!"
        );

        ll.pop_front();
        // assert_eq!(
        //     format!("{}", ll),
        //     "5".to_string(),
        //     "Doubly Linked List is invalid after second front pop!"
        // );
        assert_eq!(
            ll.length, 1,
            "Doubly Linked List has invalid length after second front pop!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().data_as_ref(),
            &5,
            "Doubly Linked List has invalid Head after second front pop!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().previous_as_ref().is_none(),
            true,
            "Head has invalid previous Node after second front pop!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().is_none(),
            true,
            "Head has invalid None next Node after second front pop!"
        );

        ll.pop_front();
        // assert_eq!(
        //     format!("{}", ll),
        //     "None",
        //     "Doubly Linked List is invalid after last front pop!"
        // );
        assert_eq!(
            ll.length, 0,
            "Doubly Linked List has invalid length after last front pop!"
        );
        assert_eq!(
            ll.head_as_ref().is_none(),
            true,
            "Doubly Linked List has invalid Head after last front pop!"
        );
        assert_eq!(
            ll.tail_as_ref().is_none(),
            true,
            "Doubly Linked List has invalid Tail after last front pop!"
        )
    }

    #[test]
    fn test_pop_back() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        ll.push_front(15);

        ll.pop_back();
        // assert_eq!(
        //     format!("{}", ll),
        //     "15 -> 10".to_string(),
        //     "Doubly Linked List is invalid after first back pop!"
        // );
        assert_eq!(
            ll.length, 2,
            "Doubly Linked List has invalid length after pop!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().data_as_ref(),
            &10,
            "Doubly Linked List has invalid Tail after first back pop!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().next_as_ref().is_none(),
            true,
            "Tail has invalid next Node after first back pop!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().previous_as_ref().is_some(),
            true,
            "Tail has invalid None previous Node after first back pop!"
        );
        assert_eq!(
            ll.tail_as_ref()
                .unwrap()
                .previous_as_ref()
                .unwrap()
                .data_as_ref(),
            &15,
            "Tail has invalid previous Node after first back pop!"
        );

        assert_eq!(
            ll.head_as_ref().unwrap().data_as_ref(),
            &15,
            "Doubly Linked List has invalid Head after back pop!"
        );

        ll.pop_back();
        // assert_eq!(
        //     format!("{}", ll),
        //     "15".to_string(),
        //     "Doubly Linked List is invalid after second back pop!"
        // );
        assert_eq!(
            ll.length, 1,
            "Doubly Linked List has invalid length after second back pop!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().data_as_ref(),
            &15,
            "Doubly Linked List has invalid Tail after second back pop!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().previous_as_ref().is_none(),
            true,
            "Tail has invalid None previous Node after second back pop!"
        );
        assert_eq!(
            ll.tail_as_ref().unwrap().next_as_ref().is_none(),
            true,
            "Tail has invalid next Node after second back pop!"
        );

        ll.pop_back();
        // assert_eq!(
        //     format!("{}", ll),
        //     "None",
        //     "Doubly Linked List is invalid after last back pop!"
        // );
        assert_eq!(
            ll.length, 0,
            "Doubly Linked List has invalid length after last back pop!"
        );
        assert_eq!(
            ll.head_as_ref().is_none(),
            true,
            "Doubly Linked List has invalid Head after last back pop!"
        );
        assert_eq!(
            ll.tail_as_ref().is_none(),
            true,
            "Doubly Linked List has invalid Tail after last back pop!"
        )
    }

    #[test]
    fn test_insert() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        ll.push_front(5);
        assert_eq!(ll.length, 1, "Linked List has invalid length after insert!");

        ll.insert(0, 7);
        assert_eq!(ll.length, 2, "Linked List has invalid length after insert!");
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().is_some(),
            true,
            "Doubly Linked List new insert has invalid next node!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .data_as_ref(),
            &5,
            "Doubly Linked List new insert has invalid next node value!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .previous_as_ref()
                .unwrap()
                .data_as_ref(),
            &7,
            "Doubly Linked List shifted node after insert has invalid previous node value!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().previous_as_ref().is_none(),
            true,
            "Doubly Linked List new insert has invalid previous node!"
        );

        ll.insert(1, 6);
        assert_eq!(ll.length, 3, "Linked List has invalid length after insert!");
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().unwrap().data_as_ref(),
            &6,
            "Doubly Linked List newly inserted node's previous node is pointing to an invalid value!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .previous_as_ref()
                .is_some(),
            true,
            "Doubly Linked List new insert has invalid previous node!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .previous_as_ref()
                .unwrap()
                .data_as_ref(),
            &7,
            "Doubly Linked List new insert has invalid previous node value!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .data_as_ref(),
            &6,
            "Doubly Linked List new insert has invalid value!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .next_as_ref()
                .is_some(),
            true,
            "Doubly Linked List new insert has invalid next node!"
        );
        assert_eq!(
            ll.head_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .next_as_ref()
                .unwrap()
                .data_as_ref(),
            &5,
            "Doubly Linked List new insert has invalid next node value!"
        );
        assert_eq!(
            ll.tail_as_ref()
                .unwrap()
                .previous_as_ref()
                .unwrap()
                .data_as_ref(),
            &6,
            "Doubly Linked List newly inserted node's next node is pointing to an invalid value!"
        );
    }

    #[test]
    #[should_panic(expected = "Index out of bounds!")]
    fn test_insert_panic() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        ll.insert(5, 5);
    }

    #[test]
    fn test_delete() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        // ll.push_front(5);
        ll.push_front(10);
        ll.push_front(15);
        ll.push_front(20);
        ll.push_front(25);

        ll.delete(0);
        assert_eq!(ll.length, 3, "Linked List has invalid length after delete!");
        assert_eq!(
            ll.head_as_ref().is_some(),
            true,
            "Doubly Linked List must have a new head after head delete!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().data_as_ref(),
            &20,
            "Doubly Linked List new head has invalid data after delete!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().previous_as_ref().is_none(),
            true,
            "Doubly Linked List new head must not have any previous node!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().is_some(),
            true,
            "Doubly Linked List new head must have next node here!"
        );
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().unwrap().data_as_ref(),
            &15,
            "Doubly Linked List new head must have next node here!"
        );
        println!("{}", ll);
        ll.delete(1);
        assert_eq!(ll.length, 2, "Linked List has invalid length after delete!");
        assert_eq!(
            ll.head_as_ref().unwrap().data_as_ref(),
            &20,
            "Doubly Linked List must have the same head delete!"
        );
        println!("{}", ll);
        assert_eq!(
            ll.head_as_ref().unwrap().next_as_ref().unwrap().data_as_ref(),
            &10,
            "Doubly Linked List head must have new node after delete!"
        );

        // assert_eq!(
        //     ll.head_as_ref()
        //         .unwrap()
        //         .next_as_ref()
        //         .unwrap()
        //         .data_as_ref(),
        //     &5,
        //     "Doubly Linked List new insert has invalid next node value!"
        // );

        // assert_eq!(
        //     format!("{}", ll),
        //     "20 -> 15 -> 10 -> 5".to_string(),
        //     "Linked List is invalid after first index deletion!"
        // );
        // assert_eq!(
        //     ll.head_as_ref().unwrap().data,
        //     20,
        //     "Linked List has invalid Head after first index deletion!"
        // );
        //
        // ll.delete(3);
        // assert_eq!(
        //     format!("{}", ll),
        //     "20 -> 15 -> 10".to_string(),
        //     "Linked List is invalid after last index deletion!"
        // );
        // assert_eq!(ll.length, 3, "Linked List has invalid length after delete!");
        // assert_eq!(
        //     ll.tail_as_ref().unwrap().data,
        //     10,
        //     "Linked List has invalid Tail after last index deletion!"
        // );
        //
        // ll.delete(2);
        // assert_eq!(
        //     format!("{}", ll),
        //     "20 -> 15".to_string(),
        //     "Linked List is invalid after middle index deletion!"
        // );
        // assert_eq!(ll.length, 2, "Linked List has invalid length after delete!");
        // assert_eq!(
        //     ll.tail_as_ref().unwrap().data,
        //     15,
        //     "Linked List has invalid Tail after middle deletion!"
        // );
        //
        // ll.delete(1);
        // ll.delete(0);
        // assert_eq!(
        //     format!("{}", ll),
        //     "None".to_string(),
        //     "Linked List is invalid after all indices deletion!"
        // );
        // assert_eq!(ll.length, 0, "Linked List has invalid length after delete!");
        // assert_eq!(
        //     ll.head_as_ref().is_none(),
        //     true,
        //     "Linked List has invalid Head for empty List!"
        // );
        // assert_eq!(
        //     ll.tail_as_ref().is_none(),
        //     true,
        //     "Linked List has invalid Tail for empty List!"
        // );
    }
    //
    // #[test]
    // #[should_panic(expected = "Index out of bounds!")]
    // fn test_delete_panic() {
    //     let mut ll: LinkedList<u8> = LinkedList::new();
    //     ll.delete(0);
    // }

    // #[test]
    // fn test_swap() {
    //     let mut ll: LinkedList<u8> = LinkedList::new();
    //     ll.push_front(1);
    //     ll.push_front(2);
    //     ll.push_front(3);
    //     ll.push_front(4);
    //
    //     assert_eq!(
    //         ll.swap(5, 10).unwrap_err(),
    //         "Index out of bounds!",
    //         "Linked List invalid indices exception failed!"
    //     );
    //
    //     ll.swap(1, 2).expect("Failed to swap Nodes");
    //     assert_eq!(
    //         format!("{}", ll),
    //         "4 -> 2 -> 3 -> 1".to_string(),
    //         "Linked List is invalid after swapping middle Nodes!"
    //     );
    //
    //     ll.swap(2, 3).expect("Failed to swap Nodes");
    //     assert_eq!(
    //         format!("{}", ll),
    //         "4 -> 2 -> 1 -> 3".to_string(),
    //         "Linked List is invalid after swapping last Nodes!"
    //     );
    //
    //     ll.swap(0, 1).expect("Failed to swap Nodes");
    //     assert_eq!(
    //         format!("{}", ll),
    //         "2 -> 4 -> 1 -> 3".to_string(),
    //         "Linked List is invalid after swapping initial Nodes!"
    //     );
    // }

    #[test]
    fn test_len() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        assert_eq!(ll.len(), 2, "Doubly Linked List invalid nodes length!");
        ll.pop_front();
        assert_eq!(ll.len(), 1, "Doubly Linked List invalid nodes length!")
    }

    #[test]
    fn test_peek() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        ll.push_front(5);
        ll.push_front(10);
        assert_eq!(
            ll.peek(0),
            &10u8,
            "Doubly Linked List invalid value at 0 index!"
        );
        assert_eq!(
            ll.peek(1),
            &5u8,
            "Doubly Linked List invalid value at 1 index!"
        );
    }

    #[test]
    #[should_panic(expected = "Index out of bounds!")]
    fn test_peek_panic() {
        let ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        ll.peek(0);
    }

    #[test]
    fn test_peek_mut() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        ll.push_front(10);
        assert_eq!(
            ll.peek_mut(0),
            &mut 10u8,
            "Doubly Linked List invalid head value!"
        );
    }

    #[test]
    #[should_panic(expected = "Index out of bounds!")]
    fn test_peek_mut_panic() {
        let mut ll: DoublyLinkedList<u8> = DoublyLinkedList::new();
        ll.peek_mut(0);
    }

    // #[test]
    // fn test_sum() {
    //     let mut ll: LinkedList<u8> = LinkedList::new();
    //     ll.push_front(5);
    //     ll.push_front(10);
    //     assert_eq!(ll.sum(), 15, "Invalid Doubly Linked List sum!");
    // }
    //
    // #[test]
    // fn test_max() {
    //     let mut ll: LinkedList<i8> = LinkedList::new();
    //     ll.push_front(5);
    //     ll.push_front(10);
    //     ll.push_front(2);
    //     ll.push_front(-5);
    //     assert_eq!(ll.max(), 10, "Invalid Doubly Linked List max value!");
    // }
    //
    // #[test]
    // fn test_min() {
    //     let mut ll: LinkedList<i8> = LinkedList::new();
    //     ll.push_front(5);
    //     ll.push_front(10);
    //     ll.push_front(2);
    //     ll.push_front(-5);
    //     assert_eq!(ll.min(), -5, "Invalid Doubly Linked List min value!");
    // }
    //
    // #[test]
    // fn test_linear_search() {
    //     let mut ll: LinkedList<u8> = LinkedList::new();
    //     ll.push_front(5);
    //     ll.push_front(10);
    //     ll.push_front(2);
    //     assert_eq!(
    //         ll.linear_search(10),
    //         ll.head_as_ref().as_ref().unwrap().next.as_ref(),
    //         "Invalid Linked List node for searchable value!"
    //     );
    //     assert_eq!(
    //         ll.linear_search(11),
    //         None,
    //         "Invalid Linked List node for unsearchable value!"
    //     );
    // }
    //
    // #[test]
    // fn test_move_to_head_search() {
    //     let mut ll: LinkedList<u8> = LinkedList::new();
    //     ll.push_front(5);
    //     ll.push_front(10);
    //     ll.push_front(2);
    //
    //     let node_1 = ll.linear_search(5).unwrap().clone();
    //     let node_2 = ll.linear_search(10).unwrap().clone();
    //     let node_3 = ll.linear_search(2).unwrap().clone();
    //     // println!("Node 1 (5) = {:?}", node_1);
    //     // println!("Node 2 (10) = {:?}", node_2);
    //     // println!("Node 3 (2) = {:?}", node_3);
    //
    //     // Original List: 2 -> 10 -> 5
    //     // Moved List: 10 -> 2 -> 5
    //
    //     // Try moving some in between Node
    //     assert_eq!(
    //         *ll.move_to_head_search(10).unwrap(),
    //         node_2,
    //         "Failed to return the found Node!"
    //     );
    //     assert_eq!(
    //         ll.head_previous_as_ref().is_none(),
    //         true,
    //         "Moved Node has invalid previous Node set!"
    //     );
    //     assert_eq!(
    //         *ll.head_next_as_ref().unwrap(),
    //         node_3,
    //         "Moved Node has invalid next Node set!"
    //     );
    //     assert_eq!(
    //         *node_3.next_as_ref().unwrap(),
    //         node_1,
    //         "Old previous Node has invalid adjusted next Node set!"
    //     );
    //     assert_eq!(
    //         *node_1.previous_as_ref().unwrap(),
    //         node_3,
    //         "Old next Node has invalid adjusted previous Node set!"
    //     );
    //
    //     // Try moving the already moved Node or head Node.
    //     assert_eq!(
    //         *ll.move_to_head_search(10).unwrap(),
    //         node_2,
    //         "Failed to return the found Node!"
    //     );
    //     assert_eq!(
    //         ll.head_previous_as_ref().is_none(),
    //         true,
    //         "Unmoved Node has invalid previous Node set!"
    //     );
    //     assert_eq!(
    //         *ll.head_next_as_ref().unwrap(),
    //         node_3,
    //         "Unmoved Node has invalid next Node set!"
    //     );
    //     println!("{}", ll);
    //     // Try moving the last Node
    //     println!("F Node = {:?}", node_2.next_as_ref().unwrap());
    //     assert_eq!(
    //         *ll.move_to_head_search(5).unwrap(),
    //         node_1,
    //         "Failed to return the found last Node!"
    //     );
    //     assert_eq!(
    //         node_3.next_as_ref().is_none(),
    //         true,
    //         "Last moved Node has invalid next Node set!"
    //     );
    // }
    //
    // #[test]
    // fn test_display_trait() {
    //     let mut ll: LinkedList<u8> = LinkedList::new();
    //     ll.push_front(5);
    //     ll.push_front(10);
    //     assert_eq!(
    //         format!("{}", ll),
    //         format!("10 -> 5"),
    //         "Linked List has invalid Display trait!"
    //     );
    // }
    //
    // #[test]
    // fn test_from_trait_heap_array() {
    //     let arr: HeapArray<u8> = HeapArray::values(&[1, 2, 3]);
    //     let ll: LinkedList<u8> = LinkedList::from(arr);
    //     assert_eq!(
    //         format!("{}", ll),
    //         "1 -> 2 -> 3".to_string(),
    //         "Linked List is invalid after conversion from Heap Array!"
    //     );
    // }
}
