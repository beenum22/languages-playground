// ---------
// NOTE: Deprecated Linked List code. Remove later.
// type Link<T> = Option<AtomicReferenceCounter<NodeDeprecated<T>>>;
//
// #[derive(PartialEq, Clone)]
// pub struct NodeDeprecated<T> {
//     next: Link<T>,
//     previous: Link<T>,
//     data: T,
// }
//
// impl<T> NodeDeprecated<T> {
//     pub fn next_as_ref(&self) -> Option<&AtomicReferenceCounter<NodeDeprecated<T>>> {
//         self.next.as_ref()
//     }
//
//     pub fn next_as_mut(&mut self) -> Option<&mut AtomicReferenceCounter<NodeDeprecated<T>>> {
//         self.next.as_mut()
//     }
//
//     pub fn previous_as_ref(&self) -> Option<&AtomicReferenceCounter<NodeDeprecated<T>>> {
//         self.previous.as_ref()
//     }
//
//     pub fn previous_as_mut(&mut self) -> Option<&mut AtomicReferenceCounter<NodeDeprecated<T>>> {
//         self.previous.as_mut()
//     }
//
//     pub fn get_data(&mut self) -> &T {
//         &self.data
//     }
//
//     pub fn set_next(&mut self, link: Link<T>) -> &Link<T> {
//         self.next = link;
//         &self.next
//     }
//
//     pub fn set_previous(&mut self, link: Link<T>) -> &Link<T> {
//         self.previous = link;
//         &self.previous
//     }
// }
//
// impl<T: Display> Display for NodeDeprecated<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} ", self.data)?;
//         Ok(())
//     }
// }
//
// impl<T: Debug> Debug for NodeDeprecated<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("Node")
//             .field("data", &self.data)
//             .field("next", &self.next.as_ref().map(|node| node.as_ptr()))
//             .field(
//                 "previous",
//                 &self.previous.as_ref().map(|node| node.as_ptr()),
//             )
//             .finish()
//     }
// }
//
// pub struct LinkedListDeprecated<T> {
//     head: Link<T>,
//     tail: Link<T>,
//     length: usize,
// }
//
// impl<T> LinkedListDeprecated<T> {
//     pub fn new() -> Self {
//         Self {
//             head: None,
//             tail: None,
//             length: 0,
//         }
//     }
//
//     pub fn head_as_ref(&self) -> Option<&AtomicReferenceCounter<NodeDeprecated<T>>> {
//         self.head.as_ref()
//     }
//
//     pub fn head_as_mut(&mut self) -> Option<&mut AtomicReferenceCounter<NodeDeprecated<T>>> {
//         self.head.as_mut()
//     }
//
//     pub fn tail_as_ref(&self) -> Option<&AtomicReferenceCounter<NodeDeprecated<T>>> {
//         self.tail.as_ref()
//     }
//
//     pub fn tail_as_mut(&mut self) -> Option<&mut AtomicReferenceCounter<NodeDeprecated<T>>> {
//         self.tail.as_mut()
//     }
//
//     pub fn peek(&self, index: usize) -> Option<&T> {
//         if index >= self.length {
//             panic!("Index out of bounds!");
//         }
//         let len = self.length;
//         let mut current = self.head_as_ref();
//         for i in 0..len {
//             if i == index {
//                 return current.map(|node| &node.data);
//             }
//             current = current.unwrap().next_as_ref();
//         }
//         None
//     }
//
//     pub fn peek_mut(&mut self, index: usize) -> Option<&mut T> {
//         if index >= self.length {
//             panic!("Index out of bounds!");
//         }
//         let len = self.length;
//         let mut current = self.head_as_mut();
//         for i in 0..len {
//             if i == index {
//                 return current.map(|node| &mut node.data);
//             }
//             current = current.unwrap().next_as_mut();
//         }
//         None
//     }
//
//     pub fn len(&self) -> usize {
//         return self.length;
//     }
//
//     pub fn head_next_as_ref(&self) -> Option<&AtomicReferenceCounter<NodeDeprecated<T>>> {
//         match self.head.as_ref() {
//             Some(head) => head.next_as_ref(),
//             None => None,
//         }
//     }
//
//     pub fn head_previous_as_ref(&self) -> Option<&AtomicReferenceCounter<NodeDeprecated<T>>> {
//         match self.head.as_ref() {
//             Some(head) => head.previous_as_ref(),
//             None => None,
//         }
//     }
//
//     // Time Complexity is O(1)
//     pub fn push_front(&mut self, data: T) -> () {
//         let mut new_node = AtomicReferenceCounter::new(NodeDeprecated {
//             next: None,
//             previous: None,
//             data,
//         });
//         match self.head.take() {
//             Some(mut node) => {
//                 node.set_previous(Some(new_node.clone()));
//                 new_node.set_next(Some(node));
//             }
//             None => {
//                 self.tail = Some(new_node.clone());
//             }
//         };
//         self.head = Some(new_node.clone());
//         self.length += 1;
//     }
//
//     // Time Complexity is O(1)
//     pub fn push_back(&mut self, data: T) -> () {
//         let mut new_node = AtomicReferenceCounter::new(NodeDeprecated {
//             next: None,
//             previous: None,
//             data,
//         });
//         match self.tail.take() {
//             Some(mut node) => {
//                 node.set_next(Some(new_node.clone()));
//                 new_node.set_previous(Some(node));
//             }
//             None => {
//                 self.head = Some(new_node.clone());
//             }
//         };
//         self.tail = Some(new_node.clone());
//         self.length += 1;
//     }
//
//     // Time Complexity is O(1)
//     pub fn pop_front(&mut self) -> Option<T>
//     where
//         T: Copy,
//     {
//         match self.head.take() {
//             Some(mut node) => {
//                 let data_copy = node.data;
//                 self.head = node.next.take();
//                 if self.head_as_ref().is_some() {
//                     self.head_as_mut().unwrap().set_previous(None);
//                 }
//                 self.length -= 1;
//                 if self.length == 0 {
//                     self.tail = None;
//                 }
//                 Some(data_copy)
//             }
//             None => None,
//         }
//     }
//
//     // Time Complexity is O(1)
//     pub fn pop_back(&mut self) -> Option<T>
//     where
//         T: Copy,
//     {
//         match self.tail.take() {
//             Some(mut node) => {
//                 let data_copy = node.data;
//                 self.tail = node.previous.take();
//                 if self.tail_as_ref().is_some() {
//                     self.tail_as_mut().unwrap().set_next(None);
//                 }
//                 self.length -= 1;
//                 if self.length == 0 {
//                     self.head = None;
//                 }
//                 Some(data_copy)
//             }
//             None => None,
//         }
//     }
//
//     fn insert_before_node(&mut self, mut node: Link<T>, data: T) -> Link<T>
//     where
//         T: Debug,
//     {
//         let mut new_node = AtomicReferenceCounter::new(NodeDeprecated {
//             next: None,
//             previous: None,
//             data,
//         });
//         if node.is_none() {
//             if node.is_none() {
//                 // panic!("Node to insert after is None!");
//                 self.head = Some(new_node.clone());
//                 self.tail = Some(new_node.clone());
//                 self.length += 1;
//                 return Some(new_node);
//             }
//         }
//         match node.as_mut().unwrap().previous_as_mut() {
//             Some(prev_node) => {
//                 prev_node.set_next(Some(new_node.clone())); // Set current->prev->next to new_node
//                 new_node.set_previous(Some(prev_node.clone()));
//             }
//             None => {
//                 new_node.set_previous(None);
//                 self.head = Some(new_node.clone());
//             }
//         }
//         new_node.set_next(node.clone());
//         node.unwrap().set_previous(Some(new_node.clone()));
//         self.length += 1;
//         Some(new_node)
//     }
//
//     fn insert_after_node(&mut self, mut node: Link<T>, data: T) -> Link<T> {
//         let mut new_node = AtomicReferenceCounter::new(NodeDeprecated {
//             next: None,
//             previous: None,
//             data,
//         });
//         if node.is_none() {
//             // panic!("Node to insert after is None!");
//             self.head = Some(new_node.clone());
//             self.tail = Some(new_node.clone());
//             self.length += 1;
//             return Some(new_node);
//         }
//         match node.as_mut().unwrap().next_as_mut() {
//             Some(next_node) => {
//                 next_node.set_previous(Some(new_node.clone()));
//                 new_node.set_next(Some(next_node.clone()));
//             }
//             None => {
//                 new_node.set_next(None);
//                 self.tail = Some(new_node.clone());
//             }
//         };
//         new_node.set_previous(node.clone());
//         node.unwrap().set_next(Some(new_node.clone()));
//
//         self.length += 1;
//         Some(new_node)
//     }
//
//     // Time Complexity is O(n)
//     pub fn insert(&mut self, index: usize, data: T) -> ()
//     where
//         T: Debug,
//     {
//         if index > self.length {
//             panic!("Index out of bounds! We can only insert at existing indices or after the last node.");
//         }
//         let mut current = self.head_as_ref();
//         for i in 0..self.length + 1 {
//             if i == index {
//                 let current_node = current.map(|node| node.clone());
//                 match i == self.length {
//                     false => {
//                         self.insert_before_node(current_node.clone(), data);
//                     }
//                     true => {
//                         self.insert_after_node(current_node.clone(), data);
//                     }
//                 }
//                 return;
//             }
//
//             if i < self.length - 1 {
//                 current = match current {
//                     Some(node) => node.next_as_ref(),
//                     None => current,
//                 };
//             }
//         }
//     }
//
//     // Time Complexity is O(n)
//     pub fn delete(&mut self, index: usize) -> () {
//         if index >= self.length {
//             panic!("Index out of bounds!");
//         }
//         let len = self.length;
//         let mut current = self.head_as_ref();
//         for i in 0..len {
//             if i == index {
//                 match current {
//                     Some(_) => {
//                         let mut current_clone = current.unwrap().clone();
//                         if current_clone.previous_as_ref().is_none() {
//                             if current_clone.next_as_ref().is_some() {
//                                 current_clone.next_as_mut().unwrap().set_previous(None);
//                             }
//                             self.head = current_clone.next.clone()
//                         }
//                         if current_clone.next_as_ref().is_none() {
//                             if current_clone.previous_as_ref().is_none() {
//                                 self.head = None;
//                                 self.tail = None;
//                             } else {
//                                 current_clone.previous_as_mut().unwrap().set_next(None);
//                                 self.tail = current_clone.previous.clone();
//                             }
//                         }
//                     }
//                     None => (),
//                 }
//                 self.length -= 1;
//                 break;
//             }
//             current = current.unwrap().next_as_ref();
//         }
//     }
//
//     fn swap_nodes(mut node_1: Link<T>, mut node_2: Link<T>) {
//         if node_1.is_none() || node_2.is_none() {
//             panic!("None Nodes cannot be swapped!")
//         }
//
//         node_1
//             .as_mut()
//             .unwrap()
//             .set_next(node_2.as_mut().unwrap().next.clone());
//         node_2
//             .as_mut()
//             .unwrap()
//             .set_previous(node_1.as_mut().unwrap().previous.clone());
//
//         if node_1.as_ref().unwrap().previous_as_ref().is_some() {
//             node_1
//                 .as_mut()
//                 .unwrap()
//                 .previous_as_mut()
//                 .unwrap()
//                 .set_next(node_2.clone());
//         }
//
//         if node_2.as_ref().unwrap().next_as_ref().is_some() {
//             node_2
//                 .as_mut()
//                 .unwrap()
//                 .next_as_mut()
//                 .unwrap()
//                 .set_previous(node_1.clone());
//         }
//
//         node_2.as_mut().unwrap().set_next(node_1.clone());
//         node_1.as_mut().unwrap().set_previous(node_2.clone());
//     }
//
//     // Time Complexity: Min=O(2), Max=O(n)
//     pub fn swap(&mut self, left: usize, right: usize) -> Result<(), &'static str> {
//         if left >= self.length || right >= self.length {
//             return Err("Index out of bounds!");
//         } else if left > right {
//             return Err("Left index must be greater than right index!");
//         }
//         if left == right {
//             return Ok(());
//         }
//
//         let mut current = self.head_as_ref();
//         let mut left_node: Option<AtomicReferenceCounter<NodeDeprecated<T>>> = None;
//         let mut right_node: Option<AtomicReferenceCounter<NodeDeprecated<T>>> = None;
//
//         for i in 0..self.length {
//             if i == left {
//                 left_node = Some(current.unwrap().clone());
//             } else if i == right {
//                 right_node = Some(current.unwrap().clone());
//             }
//
//             if left_node.is_some() && right_node.is_some() {
//                 if left_node.as_ref().unwrap().previous_as_ref().is_none() {
//                     self.head = right_node.clone();
//                 }
//                 if right_node.as_ref().unwrap().next_as_ref().is_none() {
//                     self.tail = left_node.clone();
//                 }
//                 Self::swap_nodes(left_node, right_node);
//                 return Ok(());
//             }
//
//             current = current.unwrap().next_as_ref();
//         }
//         Ok(())
//     }
//
//     // Time Complexity is O(n)
//     pub fn sum(&self) -> T
//     where
//         T: Copy + AddAssign + Default,
//     {
//         let mut sum: T = T::default();
//         let mut current = self.head_as_ref();
//         while !current.is_none() {
//             sum += current.unwrap().data;
//             current = current.unwrap().next_as_ref();
//         }
//         sum
//     }
//
//     // Time Complexity is O(n)
//     pub fn max(&self) -> T
//     where
//         T: Copy + Ord + Bounded,
//     {
//         let mut max: T = T::min_value();
//         let mut current = self.head_as_ref();
//         while !current.is_none() {
//             if current.as_ref().unwrap().data > max {
//                 max = current.as_ref().unwrap().data
//             }
//             current = current.as_ref().unwrap().next_as_ref();
//         }
//         max
//     }
//
//     // Time Complexity is O(n)
//     pub fn min(&self) -> T
//     where
//         T: Copy + Ord + Bounded,
//     {
//         let mut min: T = T::max_value();
//         let mut current = self.head_as_ref();
//         while !current.is_none() {
//             if current.as_ref().unwrap().data < min {
//                 min = current.as_ref().unwrap().data
//             }
//             current = current.as_ref().unwrap().next_as_ref();
//         }
//         min
//     }
//
//     // Time Complexity is O(n)
//     pub fn linear_search(&self, val: T) -> Option<&AtomicReferenceCounter<NodeDeprecated<T>>>
//     where
//         T: Ord,
//     {
//         let mut current = self.head_as_ref();
//         while !current.is_none() {
//             if current.as_ref().unwrap().data == val {
//                 return current;
//             }
//             current = current.as_ref().unwrap().next_as_ref();
//         }
//         None
//     }
//
//     // TODO: The method looks very messy with a lot of Clones. Maybe it's fine but check other ways.
//     // Time Complexity is ..
//     pub fn move_to_head_search(&mut self, val: T) -> Option<&AtomicReferenceCounter<NodeDeprecated<T>>>
//     where
//         T: Ord,
//     {
//         let mut current = Some(self.head_as_ref().unwrap().clone());
//         while !current.is_none() {
//             if current.as_ref().unwrap().data == val {
//                 if current.as_ref() != self.head_as_ref() {
//                     let current_prev = current.as_ref().unwrap().previous.clone();
//                     let current_next = current.as_ref().unwrap().next.clone();
//                     if current.as_mut().unwrap().next_as_mut().is_some() {
//                         current
//                             .as_mut()
//                             .unwrap()
//                             .previous_as_mut()
//                             .unwrap()
//                             .set_next(current_next); // Adjust the older previous Node's next Node
//                         current
//                             .as_mut()
//                             .unwrap()
//                             .next_as_mut()
//                             .unwrap()
//                             .set_previous(current_prev); // Adjust the older next Node's previous Node
//                     } else {
//                         current
//                             .as_mut()
//                             .unwrap()
//                             .previous_as_mut()
//                             .unwrap()
//                             .set_next(None); // Adjust the older previous Node's next Node
//                     }
//
//                     current.as_mut().unwrap().set_previous(None); // Set previous Node to None since it's the new head Node
//                     current
//                         .as_mut()
//                         .unwrap()
//                         .set_next(Some(self.head_as_mut().unwrap().clone())); // Set next Node to old head Node
//
//                     self.head = Some(current.unwrap().clone()); // Update head Node to the newly found Node
//                 }
//                 return self.head_as_ref();
//             }
//             current = Some(current.unwrap().next_as_mut().unwrap().clone());
//         }
//         None
//     }
//
//     pub fn sort(&mut self) -> () {}
// }
//
// impl<T: Display> Display for LinkedListDeprecated<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut current = self.head_as_ref();
//         if current.is_none() {
//             write!(f, "None")?;
//         }
//         while !current.is_none() {
//             write!(f, "{}", current.as_ref().unwrap().data)?;
//             current = current.as_ref().unwrap().next_as_ref();
//             if current.is_some() {
//                 write!(f, " -> ")?;
//             }
//         }
//         Ok(())
//     }
// }
//
// impl<T: Debug> Debug for LinkedListDeprecated<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         let mut debug_struct = f.debug_struct("LinkedList");
//         let mut debug_list = Vec::new();
//         let mut current = self.head_as_ref();
//         while !current.is_none() {
//             debug_list.push(current.unwrap());
//             current = current.as_ref().unwrap().next_as_ref();
//         }
//         debug_struct.field("Length", &self.length);
//         debug_struct.field("Nodes", &debug_list);
//         debug_struct.finish()
//     }
// }
//
// impl<T: Default> From<HeapArray<T>> for LinkedListDeprecated<T> {
//     fn from(mut value: HeapArray<T>) -> Self {
//         let mut ll: LinkedListDeprecated<T> = LinkedListDeprecated::new();
//         for _i in 0..value.get_len() {
//             ll.push_front(value.pop().unwrap())
//         }
//         ll
//     }
// }
//
// impl<T> Drop for LinkedListDeprecated<T> {
//     fn drop(&mut self) {
//         let mut current = self.head.take();
//         for _i in 0..self.length {
//             match current {
//                 Some(mut node) => {
//                     current = node.next.take();
//                 }
//                 None => return (),
//             }
//         }
//     }
// }

// ---------
// NOTE: Deprecated Linked List code. Remove later.
// #[cfg(test)]
// mod node_deprecated {
//     use crate::structs::linked_lists::{Link, NodeDeprecated};
//     use crate::structs::smart_ptrs::AtomicReferenceCounter;
//
//     #[test]
//     fn test_set_next() {
//         let mut node: NodeDeprecated<u8> = NodeDeprecated {
//             next: None,
//             previous: None,
//             data: 1u8,
//         };
//
//         let next_link: Link<u8> = Some(AtomicReferenceCounter::new(NodeDeprecated {
//             next: None,
//             previous: None,
//             data: 1u8,
//         }));
//         let next_link_ptr = next_link.as_ref().unwrap().as_ptr();
//
//         node.set_next(next_link);
//         assert_eq!(
//             node.next.is_some(),
//             true,
//             "Failed to set the next link for the node!"
//         );
//         assert_eq!(
//             node.next.unwrap().as_ptr(),
//             next_link_ptr,
//             "Invalid next link ptr found for the node!"
//         );
//     }
//
//     #[test]
//     fn test_set_previous() {
//         let mut node: NodeDeprecated<u8> = NodeDeprecated {
//             next: None,
//             previous: None,
//             data: 1u8,
//         };
//
//         let prev_link: Link<u8> = Some(AtomicReferenceCounter::new(NodeDeprecated {
//             next: None,
//             previous: None,
//             data: 1u8,
//         }));
//         let prev_link_ptr = prev_link.as_ref().unwrap().as_ptr();
//
//         node.set_previous(prev_link);
//         assert_eq!(
//             node.previous.is_some(),
//             true,
//             "Failed to set the previous link for the node!"
//         );
//         assert_eq!(
//             node.previous.unwrap().as_ptr(),
//             prev_link_ptr,
//             "Invalid previous link ptr found for the node!"
//         );
//     }
// }
//
// #[cfg(test)]
// mod linked_list_deprecated {
//     use crate::structs::arrays::HeapArray;
//     use crate::structs::linked_lists::LinkedListDeprecated;
//
//     #[test]
//     fn test_new() {
//         let ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         assert_eq!(ll.head, None, "Linked List has invalid initial Head!")
//     }
//
//     #[test]
//     fn test_head_as_ref() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         assert_eq!(
//             ll.head_as_ref(),
//             None,
//             "Linked List has invalid initial Head!"
//         );
//         ll.insert(0, 1);
//         assert_eq!(
//             ll.head_as_ref().is_some(),
//             true,
//             "Linked List has None Head after push!"
//         )
//     }
//
//     #[test]
//     fn test_tail_as_ref() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         assert_eq!(
//             ll.tail_as_ref(),
//             None,
//             "Linked List has invalid initial Tail!"
//         );
//         ll.push_front(1);
//         assert_eq!(
//             ll.tail_as_ref().is_some(),
//             true,
//             "Linked List has None Tail after push!"
//         );
//     }
//
//     #[test]
//     fn test_push_front() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//
//         ll.push_front(5);
//         assert_eq!(
//             format!("{}", ll),
//             "5".to_string(),
//             "Linked List is invalid after first front push!"
//         );
//         assert_eq!(
//             ll.length, 1,
//             "Linked List has invalid length after front push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().is_some(),
//             true,
//             "Linked List has invalid None Head after first front push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().data,
//             5,
//             "Linked List has invalid Head after first front push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().previous_as_ref().is_none(),
//             true,
//             "Head has invalid previous Node after first front push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().next_as_ref().is_none(),
//             true,
//             "Head has invalid next Node after first front push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().is_some(),
//             true,
//             "Linked List has invalid None Tail after first front push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().data,
//             5,
//             "Linked List has invalid Tail after first front push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().previous_as_ref().is_none(),
//             true,
//             "Tail has invalid previous Node after first front push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().next_as_ref().is_none(),
//             true,
//             "Tail has invalid next Node after first front push!"
//         );
//
//         ll.push_front(10);
//         assert_eq!(
//             format!("{}", ll),
//             "10 -> 5".to_string(),
//             "Linked List is invalid after second front push!"
//         );
//         assert_eq!(ll.length, 2, "Linked List has invalid length after push!");
//         assert_eq!(
//             ll.head_as_ref().is_some(),
//             true,
//             "Linked List has invalid None Head after second front push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().data,
//             10,
//             "Linked List has invalid Head after second front push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().previous_as_ref().is_none(),
//             true,
//             "Head has invalid previous Node after second front push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().next_as_ref().is_some(),
//             true,
//             "Head has invalid None next Node after second front push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().next_as_ref().unwrap().data,
//             5,
//             "Head has invalid next Node after second front push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().is_some(),
//             true,
//             "Linked List has invalid None Tail after second front push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().data,
//             5,
//             "Linked List has invalid Tail after second front push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().previous_as_ref().is_some(),
//             true,
//             "Tail has invalid None previous Node after second front push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().previous_as_ref().unwrap().data,
//             10,
//             "Tail has invalid previous Node after second front push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().next_as_ref().is_none(),
//             true,
//             "Tail has invalid next Node after second front push!"
//         );
//     }
//
//     #[test]
//     fn test_push_back() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//
//         ll.push_back(5);
//         assert_eq!(
//             format!("{}", ll),
//             "5".to_string(),
//             "Linked List is invalid after first back push!"
//         );
//         assert_eq!(
//             ll.length, 1,
//             "Linked List has invalid length after back push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().is_some(),
//             true,
//             "Linked List has invalid None Head after first back push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().data,
//             5,
//             "Linked List has invalid Head after first back push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().previous_as_ref().is_none(),
//             true,
//             "Head has invalid previous Node after first back push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().next_as_ref().is_none(),
//             true,
//             "Head has invalid next Node after first back push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().is_some(),
//             true,
//             "Linked List has invalid None Tail after first back push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().data,
//             5,
//             "Linked List has invalid Tail after first back push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().previous_as_ref().is_none(),
//             true,
//             "Tail has invalid previous Node after first back push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().next_as_ref().is_none(),
//             true,
//             "Tail has invalid next Node after first back push!"
//         );
//
//         ll.push_back(10);
//         assert_eq!(
//             format!("{}", ll),
//             "5 -> 10".to_string(),
//             "Linked List is invalid after second back push!"
//         );
//         assert_eq!(ll.length, 2, "Linked List has invalid length after push!");
//         assert_eq!(
//             ll.head_as_ref().is_some(),
//             true,
//             "Linked List has invalid None Head after second back push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().data,
//             5,
//             "Linked List has invalid Head after second back push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().previous_as_ref().is_none(),
//             true,
//             "Head has invalid previous Node after second back push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().next_as_ref().is_some(),
//             true,
//             "Head has invalid None next Node after second back push!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().next_as_ref().unwrap().data,
//             10,
//             "Head has invalid next Node after second back push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().is_some(),
//             true,
//             "Linked List has invalid None Tail after second back push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().data,
//             10,
//             "Linked List has invalid Tail after second back push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().previous_as_ref().is_some(),
//             true,
//             "Tail has invalid None previous Node after second back push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().previous_as_ref().unwrap().data,
//             5,
//             "Tail has invalid previous Node after second back push!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().next_as_ref().is_none(),
//             true,
//             "Tail has invalid next Node after second back push!"
//         );
//     }
//
//     #[test]
//     fn test_pop_front() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.push_front(5);
//         ll.push_front(10);
//         ll.push_front(15);
//
//         ll.pop_front();
//         assert_eq!(
//             format!("{}", ll),
//             "10 -> 5".to_string(),
//             "Linked List is invalid after first front pop!"
//         );
//         assert_eq!(ll.length, 2, "Linked List has invalid length after pop!");
//         assert_eq!(
//             ll.head_as_ref().unwrap().data,
//             10,
//             "Linked List has invalid Head after first front pop!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().previous_as_ref().is_none(),
//             true,
//             "Head has invalid previous Node after first front pop!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().next_as_ref().is_some(),
//             true,
//             "Head has invalid None next Node after first front pop!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().next_as_ref().unwrap().data,
//             5,
//             "Head has invalid next Node after first front pop!"
//         );
//
//         assert_eq!(
//             ll.tail_as_ref().unwrap().data,
//             5,
//             "Linked List has invalid Tail after front pop!"
//         );
//
//         ll.pop_front();
//         assert_eq!(
//             format!("{}", ll),
//             "5".to_string(),
//             "Linked List is invalid after second front pop!"
//         );
//         assert_eq!(
//             ll.length, 1,
//             "Linked List has invalid length after second front pop!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().data,
//             5,
//             "Linked List has invalid Head after second front pop!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().previous_as_ref().is_none(),
//             true,
//             "Head has invalid previous Node after second front pop!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().next_as_ref().is_none(),
//             true,
//             "Head has invalid None next Node after second front pop!"
//         );
//
//         ll.pop_front();
//         assert_eq!(
//             format!("{}", ll),
//             "None",
//             "Linked List is invalid after last front pop!"
//         );
//         assert_eq!(
//             ll.length, 0,
//             "Linked List has invalid length after last front pop!"
//         );
//         assert_eq!(
//             ll.head_as_ref().is_none(),
//             true,
//             "Linked List has invalid Head after last front pop!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().is_none(),
//             true,
//             "Linked List has invalid Tail after last front pop!"
//         )
//     }
//
//     #[test]
//     fn test_pop_back() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.push_front(5);
//         ll.push_front(10);
//         ll.push_front(15);
//
//         ll.pop_back();
//         assert_eq!(
//             format!("{}", ll),
//             "15 -> 10".to_string(),
//             "Linked List is invalid after first back pop!"
//         );
//         assert_eq!(ll.length, 2, "Linked List has invalid length after pop!");
//         assert_eq!(
//             ll.tail_as_ref().unwrap().data,
//             10,
//             "Linked List has invalid Tail after first back pop!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().next_as_ref().is_none(),
//             true,
//             "Tail has invalid next Node after first back pop!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().previous_as_ref().is_some(),
//             true,
//             "Tail has invalid None previous Node after first back pop!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().previous_as_ref().unwrap().data,
//             15,
//             "Tail has invalid previous Node after first back pop!"
//         );
//
//         assert_eq!(
//             ll.head_as_ref().unwrap().data,
//             15,
//             "Linked List has invalid Head after back pop!"
//         );
//
//         ll.pop_back();
//         assert_eq!(
//             format!("{}", ll),
//             "15".to_string(),
//             "Linked List is invalid after second back pop!"
//         );
//         assert_eq!(
//             ll.length, 1,
//             "Linked List has invalid length after second back pop!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().data,
//             15,
//             "Linked List has invalid Tail after second back pop!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().previous_as_ref().is_none(),
//             true,
//             "Tail has invalid None previous Node after second back pop!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().next_as_ref().is_none(),
//             true,
//             "Tail has invalid next Node after second back pop!"
//         );
//
//         ll.pop_back();
//         assert_eq!(
//             format!("{}", ll),
//             "None",
//             "Linked List is invalid after last back pop!"
//         );
//         assert_eq!(
//             ll.length, 0,
//             "Linked List has invalid length after last back pop!"
//         );
//         assert_eq!(
//             ll.head_as_ref().is_none(),
//             true,
//             "Linked List has invalid Head after last back pop!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().is_none(),
//             true,
//             "Linked List has invalid Tail after last back pop!"
//         )
//     }
//
//     #[test]
//     fn test_insert_before_node() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//
//         let node_1 = ll.insert_before_node(None, 5u8);
//         assert_eq!(
//             format!("{}", ll),
//             "5".to_string(),
//             "Linked List is invalid after first insert!"
//         );
//         assert_eq!(
//             ll.length, 1,
//             "Linked List has invalid length after first insert!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().as_ptr(),
//             node_1.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid Head after first insert!"
//         );
//         assert_eq!(
//             ll.head_previous_as_ref().is_none(),
//             true,
//             "Linked List has invalid previous Node after first insert!"
//         );
//         assert_eq!(
//             ll.head_next_as_ref().is_none(),
//             true,
//             "Linked List has invalid next Node after first insert!"
//         );
//
//         let node_2 = ll.insert_before_node(node_1.clone(), 10u8);
//         assert_eq!(
//             format!("{}", ll),
//             "10 -> 5".to_string(),
//             "Linked List is invalid after second insert!"
//         );
//         assert_eq!(
//             ll.length, 2,
//             "Linked List has invalid length after second insert!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().as_ptr(),
//             node_2.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid Head after second insert!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().as_ptr(),
//             node_1.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid Tail after second insert!"
//         );
//         assert_eq!(
//             ll.head_previous_as_ref().is_none(),
//             true,
//             "Linked List has invalid previous Node after second insert!"
//         );
//         assert_eq!(
//             ll.head_next_as_ref().is_some(),
//             true,
//             "Linked List has invalid next Node after second insert!"
//         );
//         assert_eq!(
//             ll.head_next_as_ref().unwrap().as_ptr(),
//             node_1.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid next Node after second insert!"
//         );
//         assert_eq!(
//             ll.head_next_as_ref().unwrap().previous_as_ref().is_some(),
//             true,
//             "Linked List has invalid previuos Node for the next Node after second insert!"
//         );
//         assert_eq!(
//             ll.head_next_as_ref()
//                 .unwrap()
//                 .previous_as_ref()
//                 .unwrap()
//                 .as_ptr(),
//             node_2.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid previuos Node for the next Node after second insert!"
//         );
//
//         let node_3 = ll.insert_before_node(node_1.clone(), 8u8);
//         assert_eq!(
//             format!("{}", ll),
//             "10 -> 8 -> 5".to_string(),
//             "Linked List is invalid after third insert!"
//         );
//         assert_eq!(
//             ll.length, 3,
//             "Linked List has invalid length after third insert!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().as_ptr(),
//             node_2.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid Head after third insert!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().as_ptr(),
//             node_1.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid Tail after third insert!"
//         );
//         assert_eq!(
//             node_2.as_ref().unwrap().next_as_ref().unwrap().as_ptr(),
//             node_3.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid previous Node after third insert!"
//         );
//         assert_eq!(
//             node_1.as_ref().unwrap().previous_as_ref().unwrap().as_ptr(),
//             node_3.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid next Node after third insert!"
//         );
//     }
//
//     #[test]
//     fn test_insert_after_node() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//
//         // ll.push_front(5u8);
//         let node_1 = ll.insert_before_node(None, 5u8);
//         assert_eq!(
//             format!("{}", ll),
//             "5".to_string(),
//             "Linked List is invalid after first insert!"
//         );
//         assert_eq!(
//             ll.length, 1,
//             "Linked List has invalid length after first insert!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().as_ptr(),
//             node_1.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid Head after first insert!"
//         );
//         assert_eq!(
//             ll.head_previous_as_ref().is_none(),
//             true,
//             "Linked List has invalid previous Node after first insert!"
//         );
//         assert_eq!(
//             ll.head_next_as_ref().is_none(),
//             true,
//             "Linked List has invalid next Node after first insert!"
//         );
//
//         let node_2 = ll.insert_after_node(node_1.clone(), 10u8);
//         assert_eq!(
//             format!("{}", ll),
//             "5 -> 10".to_string(),
//             "Linked List is invalid after second insert!"
//         );
//         assert_eq!(
//             ll.length, 2,
//             "Linked List has invalid length after second insert!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().as_ptr(),
//             node_1.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid Head after second insert!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().as_ptr(),
//             node_2.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid Tail after second insert!"
//         );
//         assert_eq!(
//             ll.head_previous_as_ref().is_none(),
//             true,
//             "Linked List Head has invalid previous Node after second insert!"
//         );
//         assert_eq!(
//             ll.head_next_as_ref().is_some(),
//             true,
//             "Linked List Head has invalid None next Node after second insert!"
//         );
//         assert_eq!(
//             ll.head_next_as_ref().unwrap().as_ptr(),
//             node_2.as_ref().unwrap().as_ptr(),
//             "Linked List Head has invalid next Node after second insert!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().next_as_ref().is_none(),
//             true,
//             "Linked List Tail has invalid next Node after second insert!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().previous_as_ref().is_some(),
//             true,
//             "Linked List Tail has invalid None previous Node after second insert!"
//         );
//         assert_eq!(
//             ll.tail_as_ref()
//                 .unwrap()
//                 .previous_as_ref()
//                 .unwrap()
//                 .as_ptr(),
//             node_1.as_ref().unwrap().as_ptr(),
//             "Linked List Tail has invalid previous Node after second insert!"
//         );
//
//         let node_3 = ll.insert_after_node(node_1.clone(), 8u8);
//         assert_eq!(
//             format!("{}", ll),
//             "5 -> 8 -> 10".to_string(),
//             "Linked List is invalid after third insert!"
//         );
//         assert_eq!(
//             ll.length, 3,
//             "Linked List has invalid length after third insert!"
//         );
//         assert_eq!(
//             ll.head_as_ref().unwrap().as_ptr(),
//             node_1.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid Head after third insert!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().unwrap().as_ptr(),
//             node_2.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid Tail after third insert!"
//         );
//         assert_eq!(
//             ll.head_next_as_ref().unwrap().as_ptr(),
//             node_3.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid next Node for Head after third insert!"
//         );
//         assert_eq!(
//             ll.tail_as_ref()
//                 .unwrap()
//                 .previous_as_ref()
//                 .unwrap()
//                 .as_ptr(),
//             node_3.as_ref().unwrap().as_ptr(),
//             "Linked List has invalid previous Node for Tail after third insert!"
//         );
//     }
//
//     #[test]
//     fn test_insert() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//
//         ll.insert(0, 5);
//         assert_eq!(
//             format!("{}", ll),
//             "5".to_string(),
//             "Linked List is invalid after insert in empty list!"
//         );
//         assert_eq!(ll.length, 1, "Linked List has invalid length after insert!");
//
//         ll.insert(0, 7);
//         assert_eq!(
//             format!("{}", ll),
//             "7 -> 5".to_string(),
//             "Linked List is invalid after insert at the start!"
//         );
//         assert_eq!(ll.length, 2, "Linked List has invalid length after insert!");
//
//         ll.insert(1, 6);
//         assert_eq!(
//             format!("{}", ll),
//             "7 -> 6 -> 5".to_string(),
//             "Linked List is invalid after insert in the middle!"
//         );
//         assert_eq!(ll.length, 3, "Linked List has invalid length after insert!");
//
//         ll.insert(3, 4);
//         assert_eq!(
//             format!("{}", ll),
//             "7 -> 6 -> 5 -> 4".to_string(),
//             "Linked List is invalid after insert next to last node!"
//         );
//         assert_eq!(ll.length, 4, "Linked List has invalid length after insert!");
//     }
//
//     #[test]
//     #[should_panic(
//         expected = "Index out of bounds! We can only insert at existing indices or after the last node."
//     )]
//     fn test_insert_panic() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.insert(5, 5);
//     }
//
//     #[test]
//     fn test_delete() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.push_front(5);
//         ll.push_front(10);
//         ll.push_front(15);
//         ll.push_front(20);
//         ll.push_front(25);
//
//         ll.delete(0);
//         assert_eq!(
//             format!("{}", ll),
//             "20 -> 15 -> 10 -> 5".to_string(),
//             "Linked List is invalid after first index deletion!"
//         );
//         assert_eq!(ll.length, 4, "Linked List has invalid length after delete!");
//         assert_eq!(
//             ll.head_as_ref().unwrap().data,
//             20,
//             "Linked List has invalid Head after first index deletion!"
//         );
//
//         ll.delete(3);
//         assert_eq!(
//             format!("{}", ll),
//             "20 -> 15 -> 10".to_string(),
//             "Linked List is invalid after last index deletion!"
//         );
//         assert_eq!(ll.length, 3, "Linked List has invalid length after delete!");
//         assert_eq!(
//             ll.tail_as_ref().unwrap().data,
//             10,
//             "Linked List has invalid Tail after last index deletion!"
//         );
//
//         ll.delete(2);
//         assert_eq!(
//             format!("{}", ll),
//             "20 -> 15".to_string(),
//             "Linked List is invalid after middle index deletion!"
//         );
//         assert_eq!(ll.length, 2, "Linked List has invalid length after delete!");
//         assert_eq!(
//             ll.tail_as_ref().unwrap().data,
//             15,
//             "Linked List has invalid Tail after middle deletion!"
//         );
//
//         ll.delete(1);
//         ll.delete(0);
//         assert_eq!(
//             format!("{}", ll),
//             "None".to_string(),
//             "Linked List is invalid after all indices deletion!"
//         );
//         assert_eq!(ll.length, 0, "Linked List has invalid length after delete!");
//         assert_eq!(
//             ll.head_as_ref().is_none(),
//             true,
//             "Linked List has invalid Head for empty List!"
//         );
//         assert_eq!(
//             ll.tail_as_ref().is_none(),
//             true,
//             "Linked List has invalid Tail for empty List!"
//         );
//     }
//
//     #[test]
//     #[should_panic(expected = "Index out of bounds!")]
//     fn test_delete_panic() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.delete(0);
//     }
//
//     #[test]
//     fn test_swap() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.push_front(1);
//         ll.push_front(2);
//         ll.push_front(3);
//         ll.push_front(4);
//
//         assert_eq!(
//             ll.swap(5, 10).unwrap_err(),
//             "Index out of bounds!",
//             "Linked List invalid indices exception failed!"
//         );
//
//         ll.swap(1, 2).expect("Failed to swap Nodes");
//         assert_eq!(
//             format!("{}", ll),
//             "4 -> 2 -> 3 -> 1".to_string(),
//             "Linked List is invalid after swapping middle Nodes!"
//         );
//
//         ll.swap(2, 3).expect("Failed to swap Nodes");
//         assert_eq!(
//             format!("{}", ll),
//             "4 -> 2 -> 1 -> 3".to_string(),
//             "Linked List is invalid after swapping last Nodes!"
//         );
//
//         ll.swap(0, 1).expect("Failed to swap Nodes");
//         assert_eq!(
//             format!("{}", ll),
//             "2 -> 4 -> 1 -> 3".to_string(),
//             "Linked List is invalid after swapping initial Nodes!"
//         );
//     }
//
//     #[test]
//     fn test_len() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.push_front(5);
//         ll.push_front(10);
//         assert_eq!(ll.len(), 2, "Linked List invalid nodes length!");
//         ll.pop_front();
//         assert_eq!(ll.len(), 1, "Linked List invalid nodes length!")
//     }
//
//     #[test]
//     fn test_peek() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.push_front(5);
//         ll.push_front(10);
//         assert_eq!(
//             ll.peek(0),
//             Some(&(10u8)),
//             "Linked List invalid value at 0 index!"
//         );
//         assert_eq!(
//             ll.peek(1),
//             Some(&(5u8)),
//             "Linked List invalid value at 1 index!"
//         );
//     }
//
//     #[test]
//     #[should_panic(expected = "Index out of bounds!")]
//     fn test_peek_panic() {
//         let ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.peek(0);
//     }
//
//     #[test]
//     fn test_peek_mut() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.push_front(10);
//         assert_eq!(
//             ll.peek_mut(0),
//             Some(&mut 10u8),
//             "Linked List invalid head value!"
//         );
//     }
//
//     #[test]
//     #[should_panic(expected = "Index out of bounds!")]
//     fn test_peek_mut_panic() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.peek_mut(0);
//     }
//
//     #[test]
//     fn test_sum() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.push_front(5);
//         ll.push_front(10);
//         assert_eq!(ll.sum(), 15, "Invalid Linked List sum!");
//     }
//
//     #[test]
//     fn test_max() {
//         let mut ll: LinkedListDeprecated<i8> = LinkedListDeprecated::new();
//         ll.push_front(5);
//         ll.push_front(10);
//         ll.push_front(2);
//         ll.push_front(-5);
//         assert_eq!(ll.max(), 10, "Invalid Linked List max value!");
//     }
//
//     #[test]
//     fn test_min() {
//         let mut ll: LinkedListDeprecated<i8> = LinkedListDeprecated::new();
//         ll.push_front(5);
//         ll.push_front(10);
//         ll.push_front(2);
//         ll.push_front(-5);
//         assert_eq!(ll.min(), -5, "Invalid Linked List min value!");
//     }
//
//     #[test]
//     fn test_linear_search() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.push_front(5);
//         ll.push_front(10);
//         ll.push_front(2);
//         assert_eq!(
//             ll.linear_search(10),
//             ll.head_as_ref().as_ref().unwrap().next.as_ref(),
//             "Invalid Linked List node for searchable value!"
//         );
//         assert_eq!(
//             ll.linear_search(11),
//             None,
//             "Invalid Linked List node for unsearchable value!"
//         );
//     }
//
//     #[test]
//     fn test_move_to_head_search() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.push_front(5);
//         ll.push_front(10);
//         ll.push_front(2);
//
//         let node_1 = ll.linear_search(5).unwrap().clone();
//         let node_2 = ll.linear_search(10).unwrap().clone();
//         let node_3 = ll.linear_search(2).unwrap().clone();
//         // println!("Node 1 (5) = {:?}", node_1);
//         // println!("Node 2 (10) = {:?}", node_2);
//         // println!("Node 3 (2) = {:?}", node_3);
//
//         // Original List: 2 -> 10 -> 5
//         // Moved List: 10 -> 2 -> 5
//
//         // Try moving some in between Node
//         assert_eq!(
//             *ll.move_to_head_search(10).unwrap(),
//             node_2,
//             "Failed to return the found Node!"
//         );
//         assert_eq!(
//             ll.head_previous_as_ref().is_none(),
//             true,
//             "Moved Node has invalid previous Node set!"
//         );
//         assert_eq!(
//             *ll.head_next_as_ref().unwrap(),
//             node_3,
//             "Moved Node has invalid next Node set!"
//         );
//         assert_eq!(
//             *node_3.next_as_ref().unwrap(),
//             node_1,
//             "Old previous Node has invalid adjusted next Node set!"
//         );
//         assert_eq!(
//             *node_1.previous_as_ref().unwrap(),
//             node_3,
//             "Old next Node has invalid adjusted previous Node set!"
//         );
//
//         // Try moving the already moved Node or head Node.
//         assert_eq!(
//             *ll.move_to_head_search(10).unwrap(),
//             node_2,
//             "Failed to return the found Node!"
//         );
//         assert_eq!(
//             ll.head_previous_as_ref().is_none(),
//             true,
//             "Unmoved Node has invalid previous Node set!"
//         );
//         assert_eq!(
//             *ll.head_next_as_ref().unwrap(),
//             node_3,
//             "Unmoved Node has invalid next Node set!"
//         );
//         println!("{}", ll);
//         // Try moving the last Node
//         println!("F Node = {:?}", node_2.next_as_ref().unwrap());
//         assert_eq!(
//             *ll.move_to_head_search(5).unwrap(),
//             node_1,
//             "Failed to return the found last Node!"
//         );
//         assert_eq!(
//             node_3.next_as_ref().is_none(),
//             true,
//             "Last moved Node has invalid next Node set!"
//         );
//     }
//
//     #[test]
//     fn test_display_trait() {
//         let mut ll: LinkedListDeprecated<u8> = LinkedListDeprecated::new();
//         ll.push_front(5);
//         ll.push_front(10);
//         assert_eq!(
//             format!("{}", ll),
//             format!("10 -> 5"),
//             "Linked List has invalid Display trait!"
//         );
//     }
//
//     #[test]
//     fn test_from_trait_heap_array() {
//         let arr: HeapArray<u8> = HeapArray::values(&[1, 2, 3]);
//         let ll: LinkedListDeprecated<u8> = LinkedListDeprecated::from(arr);
//         assert_eq!(
//             format!("{}", ll),
//             "1 -> 2 -> 3".to_string(),
//             "Linked List is invalid after conversion from Heap Array!"
//         );
//     }
// }