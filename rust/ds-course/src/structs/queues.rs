/*
Queue Data Structures
- Fixed Queues with contiguous memory using an Array data structure
    - Basic Queue with O(n) dequeue and O(1) enqueue
    - One time usable Queue with O(1) dequeue and dequeue
    - Circular Queue with O(1) enqueue and dequeue
    - Double Ended Queue (DeQueue)
        - Input Restricted
        - Output Restricted
    - Priority Queues
        - Defined priority types
        - Element itself represents priorities
- Dynamic Queues with non-contiguous memory using a Linked List data structure
*/
use crate::structs::arrays::HeapArray;
use std::fmt::{Display, Formatter};

trait QueueADT<T> {
    fn enqueue(&mut self, item: T) -> Result<(), &str>;

    fn dequeue(&mut self) -> Result<(T), &str>;

    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;
}

trait DeQueueADT<T> {
    fn enqueue_front(&mut self, item: T) -> Result<(), &str>;

    fn enqueue_back(&mut self, item: T) -> Result<(), &str>;

    fn dequeue_front(&mut self) -> Result<(T), &str>;

    fn dequeue_back(&mut self) -> Result<(T), &str>;

    fn is_empty(&self) -> bool;

    fn is_full(&self) -> bool;
}

pub struct BasicQueue<T> {
    length: usize,
    size: usize,
    data: HeapArray<T>,
}

impl<T> BasicQueue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            length: 0,
            size,
            data: HeapArray::with_capacity(size),
        }
    }
}

impl<T> QueueADT<T> for BasicQueue<T> {
    // Time Complexity is O(1).
    fn enqueue(&mut self, item: T) -> Result<(), &str> {
        if self.is_full() {
            return Err("Queue is full!");
        }
        self.data.push(item);
        self.length += 1;
        Ok(())
    }

    // Time Complexity is O(n) because Array shifts all the elements left after item is removed.
    fn dequeue(&mut self) -> Result<(T), &str> {
        if self.is_empty() {
            return Err("Queue is empty!");
        }
        let data = self.data.delete(0);
        self.length -= 1;
        Ok(data)
    }

    fn is_empty(&self) -> bool {
        if self.length == 0 {
            return true;
        }
        false
    }

    fn is_full(&self) -> bool {
        if self.length == self.size {
            return true;
        }
        false
    }
}

pub struct EphemeralQueue<T> {
    front: usize,
    rear: usize,
    length: usize,
    size: usize,
    data: HeapArray<T>,
}

impl<T> EphemeralQueue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            front: 0,
            rear: 0,
            length: 0,
            size,
            data: HeapArray::with_capacity(size),
        }
    }
}

impl<T> QueueADT<T> for EphemeralQueue<T> {
    // Time Complexity is O(1).
    fn enqueue(&mut self, item: T) -> Result<(), &str> {
        if self.is_full() {
            return Err("Queue is full!");
        }
        self.rear += 1;
        self.data.push(item);
        self.length += 1;
        Ok(())
    }

    // Time Complexity is O(1).
    fn dequeue(&mut self) -> Result<(T), &str> {
        if self.is_empty() {
            return Err("Queue is empty!");
        }
        let data = unsafe { self.data.get_copy(self.front) };
        self.front += 1;
        Ok(data)
    }

    fn is_empty(&self) -> bool {
        if self.rear == self.front {
            return true;
        }
        false
    }

    fn is_full(&self) -> bool {
        if self.rear == self.size {
            return true;
        }
        false
    }
}

#[derive(Debug)]
pub struct CircularQueue<T> {
    front: usize,
    rear: usize,
    length: usize,
    size: usize,
    data: HeapArray<T>,
}

impl<T> CircularQueue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            front: 0,
            rear: 0,
            length: 0,
            size,
            data: HeapArray::with_capacity(size),
        }
    }
}

impl<T> QueueADT<T> for CircularQueue<T> {
    // Time Complexity is O(1).
    fn enqueue(&mut self, item: T) -> Result<(), &str> {
        if self.is_full() {
            return Err("Queue is full!");
        }
        // We have to use push in this case because set operation doesn't work for arrays with no value at index.
        if self.rear >= self.front {
            self.data.push(item);
        } else {
            self.data.set(self.rear, item);
        }
        self.rear = (self.rear + 1) % self.size;
        self.length += 1;
        Ok(())
    }

    // Time Complexity is O(1).
    fn dequeue(&mut self) -> Result<(T), &str> {
        if self.is_empty() {
            return Err("Queue is empty!");
        }
        let data = unsafe { self.data.get_copy(self.front) };
        self.front = (self.front + 1) % self.size;
        self.length -= 1;
        Ok(data)
    }

    fn is_empty(&self) -> bool {
        if self.length == 0 {
            return true;
        }
        false
    }

    fn is_full(&self) -> bool {
        if self.length == self.size {
            return true;
        }
        false
    }
}

impl<T: Display> Display for CircularQueue<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut l = self.length;
        let mut i = self.front;
        while l > 0 {
            write!(f, "[{}]", self.data[i])?;
            i = (i + 1) % self.size;
            l -= 1;
        }
        for i in 0..(self.size - self.length) {
            write!(f, "[]")?;
        }
        Ok(())
    }
}

pub struct DeQueue<T> {
    front: usize,
    rear: usize,
    length: usize,
    size: usize,
    data: HeapArray<T>,
}

impl<T> DeQueue<T> {
    pub fn new(size: usize) -> Self {
        let mut arr: HeapArray<T> = HeapArray::with_capacity(size);
        Self {
            front: 0,
            rear: 0,
            length: 0,
            size,
            data: arr,
        }
    }
}

impl<T> DeQueueADT<T> for DeQueue<T> {
    fn enqueue_front(&mut self, item: T) -> Result<(), &str> {
        if self.is_full() {
            return Err("Queue is full!");
        }
        // We have to use push in this case because set operation doesn't work for arrays with no value at index.
        self.front = (self.front - 1) % self.size;
        if self.front >= self.rear {
            self.data.push(item);
        } else {
            self.data.set(self.front, item);
        }
        self.length += 1;
        Ok(())
    }

    fn enqueue_back(&mut self, item: T) -> Result<(), &str> {
        if self.is_full() {
            return Err("Queue is full!");
        }
        // We have to use push in this case because set operation doesn't work for arrays with no value at index.
        if self.rear >= self.front {
            self.data.push(item);
        } else {
            self.data.set(self.rear, item);
        }
        self.rear = (self.rear + 1) % self.size;
        self.length += 1;
        Ok(())
    }

    fn dequeue_front(&mut self) -> Result<(T), &str> {
        if self.is_empty() {
            return Err("DeQueue is empty!");
        }
        let data = unsafe { self.data.get_copy(self.front) };
        self.front = (self.front + 1) % self.size;
        self.length -= 1;
        Ok(data)
    }

    fn dequeue_back(&mut self) -> Result<(T), &str> {
        if self.is_empty() {
            return Err("DeQueue is empty!");
        }
        self.rear = (self.rear - 1) % self.size;
        let data = unsafe { self.data.get_copy(self.rear) };
        self.length -= 1;
        Ok(data)
    }

    fn is_empty(&self) -> bool {
        if self.length == 0 {
            return true;
        }
        false
    }

    fn is_full(&self) -> bool {
        if self.length == self.size {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod basic_queue {
    use crate::structs::arrays::HeapArray;
    use crate::structs::queues::{BasicQueue, QueueADT};

    #[test]
    fn test_new() {
        let queue: BasicQueue<u8> = BasicQueue::new(5);
        assert_eq!(queue.size, 5, "BasicQueue size is invalid!");
        assert_eq!(queue.length, 0, "BasicQueue length is invalid!");
        assert_eq!(queue.data, HeapArray::new(), "BasicQueue array is invalid!");
    }

    #[test]
    fn test_enqueue() {
        let mut queue: BasicQueue<u8> = BasicQueue::new(2);
        queue.enqueue(1).expect("Failed to enqueue!");
        assert_eq!(queue.length, 1, "BasicQueue length is invalid!");
        queue.enqueue(2).expect("Failed to enqueue!");
        assert_eq!(queue.length, 2, "BasicQueue length is invalid!");
        assert!(
            queue.enqueue(3).is_err(),
            "BasicQueue must throw error for being full!"
        );
    }

    #[test]
    fn test_dequeue() {
        let mut queue: BasicQueue<u8> = BasicQueue::new(2);
        queue.enqueue(1).expect("Failed to enqueue!");
        queue.enqueue(2).expect("Failed to enqueue!");
        queue.dequeue().expect("Failed to dequeue!");
        assert_eq!(queue.length, 1, "BasicQueue length is invalid!");
        queue.dequeue().expect("Failed to dequeue!");
    }

    #[test]
    fn test_is_empty() {
        let mut queue: BasicQueue<u8> = BasicQueue::new(5);
        assert_eq!(queue.is_empty(), true, "BasicQueue must be empty!");
        queue.enqueue(5).expect("Failed to enqueue!");
        assert_eq!(queue.is_empty(), false, "BasicQueue must not be empty!");
    }

    #[test]
    fn test_is_full() {
        let mut queue: BasicQueue<u8> = BasicQueue::new(2);
        queue.enqueue(5).expect("Failed to enqueue!");
        assert_eq!(queue.is_full(), false, "BasicQueue must not be full!");
        queue.enqueue(5).expect("Failed to enqueue!");
        assert_eq!(queue.is_full(), true, "BasicQueue must be full!");
    }
}

mod ephemeral_queue {
    use crate::structs::arrays::HeapArray;
    use crate::structs::queues::{EphemeralQueue, QueueADT};

    #[test]
    fn test_new() {
        let queue: EphemeralQueue<u8> = EphemeralQueue::new(5);
        assert_eq!(queue.front, 0, "EphemeralQueue front index is invalid!");
        assert_eq!(queue.rear, 0, "EphemeralQueue rear index is invalid!");
        assert_eq!(queue.size, 5, "EphemeralQueue size is invalid!");
        assert_eq!(queue.length, 0, "EphemeralQueue length is invalid!");
        assert_eq!(
            queue.data,
            HeapArray::new(),
            "EphemeralQueue array is invalid!"
        );
    }

    #[test]
    fn test_enqueue() {
        let mut queue: EphemeralQueue<u8> = EphemeralQueue::new(2);
        queue.enqueue(1).expect("Failed to enqueue!");
        assert_eq!(queue.length, 1, "EphemeralQueue length is invalid!");
        assert_eq!(queue.front, 0, "EphemeralQueue front index is invalid!");
        assert_eq!(queue.rear, 1, "EphemeralQueue rear index is invalid!");
        queue.enqueue(2).expect("Failed to enqueue!");
        assert_eq!(queue.length, 2, "EphemeralQueue length is invalid!");
        assert_eq!(queue.front, 0, "EphemeralQueue front index is invalid!");
        assert_eq!(queue.rear, 2, "EphemeralQueue rear index is invalid!");
        assert!(
            queue.enqueue(3).is_err(),
            "EphemeralQueue must throw error for being full!"
        );
    }

    #[test]
    fn test_dequeue() {
        let mut queue: EphemeralQueue<u8> = EphemeralQueue::new(2);
        queue.enqueue(1).expect("Failed to enqueue!");
        queue.enqueue(2).expect("Failed to enqueue!");
        queue.dequeue().expect("Failed to dequeue!");
        assert_eq!(queue.length, 2, "EphemeralQueue length is invalid!");
        assert_eq!(queue.front, 1, "EphemeralQueue front index is invalid!");
        assert_eq!(queue.rear, 2, "EphemeralQueue rear index is invalid!");
        queue.dequeue().expect("Failed to dequeue!");
        assert_eq!(queue.length, 2, "EphemeralQueue length is invalid!");
        assert_eq!(queue.front, 2, "EphemeralQueue front index is invalid!");
        assert_eq!(queue.rear, 2, "EphemeralQueue rear index is invalid!");
        assert!(
            queue.dequeue().is_err(),
            "EphemeralQueue must throw error for being full!"
        );
    }

    #[test]
    fn test_is_empty() {
        let mut queue: EphemeralQueue<u8> = EphemeralQueue::new(5);
        assert_eq!(queue.is_empty(), true, "EphemeralQueue must be empty!");
        queue.enqueue(5);
        assert_eq!(queue.is_empty(), false, "EphemeralQueue must not be empty!");
    }

    #[test]
    fn test_is_full() {
        let mut queue: EphemeralQueue<u8> = EphemeralQueue::new(2);
        queue.enqueue(5).expect("Error");
        assert_eq!(queue.is_full(), false, "EphemeralQueue must not be full!");
        queue.enqueue(5).expect("Error");
        assert_eq!(queue.is_full(), true, "EphemeralQueue must be full!");
    }
}

mod circular_queue {
    use crate::structs::arrays::HeapArray;
    use crate::structs::queues::{CircularQueue, QueueADT};

    #[test]
    fn test_new() {
        let queue: CircularQueue<u8> = CircularQueue::new(5);
        assert_eq!(queue.front, 0, "CircularQueue front index is invalid!");
        assert_eq!(queue.rear, 0, "CircularQueue rear index is invalid!");
        assert_eq!(queue.size, 5, "CircularQueue size is invalid!");
        assert_eq!(queue.length, 0, "CircularQueue length is invalid!");
        assert_eq!(
            queue.data,
            HeapArray::new(),
            "CircularQueue array is invalid!"
        );
    }

    #[test]
    fn test_enqueue() {
        let mut queue: CircularQueue<u8> = CircularQueue::new(3);
        queue.enqueue(1).expect("Failed to enqueue!");
        assert_eq!(queue.length, 1, "CircularQueue length is invalid!");
        assert_eq!(queue.front, 0, "CircularQueue front index is invalid!");
        assert_eq!(queue.rear, 1, "CircularQueue rear index is invalid!");
        assert_eq!(
            format!("{}", queue.data),
            "[1]",
            "CircularQueue is invalid!"
        );
        queue.enqueue(2).expect("Failed to enqueue!");
        queue.enqueue(3).expect("Failed to enqueue!");
        assert_eq!(queue.length, 3, "CircularQueue length is invalid!");
        assert_eq!(queue.front, 0, "CircularQueue front index is invalid!");
        assert_eq!(queue.rear, 0, "CircularQueue rear index is invalid!");
        assert_eq!(
            format!("{}", queue.data),
            "[1, 2, 3]",
            "CircularQueue is invalid!"
        );
        assert!(
            queue.enqueue(3).is_err(),
            "CircularQueue must throw error for being full!"
        );
        queue.dequeue().expect("Failed to dequeue!");
        queue.enqueue(4).expect("Failed to enqueue!");
        assert_eq!(queue.length, 3, "CircularQueue length is invalid!");
        assert_eq!(queue.front, 1, "CircularQueue front index is invalid!");
        assert_eq!(queue.rear, 1, "CircularQueue rear index is invalid!");
        assert_eq!(
            format!("{}", queue.data),
            "[4, 2, 3]",
            "CircularQueue is invalid!"
        );
        assert!(
            queue.enqueue(3).is_err(),
            "CircularQueue must throw error for being full!"
        );
    }

    #[test]
    fn test_dequeue() {
        let mut queue: CircularQueue<u8> = CircularQueue::new(3);
        queue.enqueue(1).expect("Failed to enqueue!");
        queue.enqueue(2).expect("Failed to enqueue!");
        assert_eq!(
            queue.dequeue().expect("Failed to dequeue!"),
            1,
            "CircularQueue dequeue returned value is invalid!"
        );
        assert_eq!(queue.length, 1, "CircularQueue length is invalid!");
        assert_eq!(queue.front, 1, "CircularQueue front index is invalid!");
        assert_eq!(queue.rear, 2, "CircularQueue rear index is invalid!");
        queue.enqueue(3).expect("Failed to enqueue!");
        queue.enqueue(4).expect("Failed to enqueue!");
        println!("{}", queue.data);
        assert_eq!(queue.front, 1, "CircularQueue front index is invalid!");
        assert_eq!(queue.rear, 1, "CircularQueue rear index is invalid!");
        assert_eq!(
            queue.dequeue().expect("Failed to dequeue!"),
            2,
            "CircularQueue dequeue returned value is invalid!"
        );
        assert_eq!(queue.length, 2, "CircularQueue length is invalid!");
        assert_eq!(queue.front, 2, "CircularQueue front index is invalid!");
        assert_eq!(queue.rear, 1, "CircularQueue rear index is invalid!");
        queue.enqueue(5).expect("Failed to enqueue!");
        queue.dequeue().expect("Failed to dequeue!");
        queue.dequeue().expect("Failed to dequeue!");
        queue.dequeue().expect("Failed to dequeue!");
        assert!(
            queue.dequeue().is_err(),
            "CircularQueue must throw error for being full!"
        );
    }

    #[test]
    fn test_is_empty() {
        let mut queue: CircularQueue<u8> = CircularQueue::new(5);
        assert_eq!(queue.is_empty(), true, "CircularQueue must be empty!");
        queue.enqueue(5);
        assert_eq!(queue.is_empty(), false, "CircularQueue must not be empty!");
    }

    #[test]
    fn test_is_full() {
        let mut queue: CircularQueue<u8> = CircularQueue::new(2);
        queue.enqueue(5).expect("Error");
        assert_eq!(queue.is_full(), false, "CircularQueue must not be full!");
        queue.enqueue(5).expect("Error");
        assert_eq!(queue.is_full(), true, "CircularQueue must be full!");
    }

    #[test]
    fn test_display_trait() {
        let mut queue: CircularQueue<u8> = CircularQueue::new(3);
        queue.enqueue(1).expect("Error");
        assert_eq!(format!("{}", queue), "[1][][]");
        queue.enqueue(2).expect("Error");
        println!("{}", queue.data);
        assert_eq!(format!("{}", queue), "[1][2][]");
        queue.enqueue(3).expect("Error");
        assert_eq!(format!("{}", queue), "[1][2][3]");
        queue.dequeue().expect("Error");
        assert_eq!(format!("{}", queue), "[2][3][]");
        queue.enqueue(4).expect("Error");
        assert_eq!(format!("{}", queue), "[2][3][4]");
        queue.dequeue().expect("Error");
        assert_eq!(format!("{}", queue), "[3][4][]");
        queue.dequeue().expect("Error");
        assert_eq!(format!("{}", queue), "[4][][]");
        queue.dequeue().expect("Error");
        assert_eq!(format!("{}", queue), "[][][]");
    }
}

mod dequeue {
    use crate::structs::arrays::HeapArray;
    use crate::structs::queues::{DeQueue, DeQueueADT};

    #[test]
    fn test_new() {
        let queue: DeQueue<u8> = DeQueue::new(5);
        assert_eq!(queue.front, 0, "DeQueue front index is invalid!");
        assert_eq!(queue.rear, 0, "DeQueue rear index is invalid!");
        assert_eq!(queue.size, 5, "DeQueue size is invalid!");
        assert_eq!(queue.length, 0, "DeQueue length is invalid!");
        assert_eq!(queue.data, HeapArray::new(), "DeQueue array is invalid!");
    }

    #[test]
    fn test_enqueue_back() {
        let mut queue: DeQueue<u8> = DeQueue::new(3);
        queue.enqueue_back(1).expect("Failed to enqueue!");
        assert_eq!(queue.length, 1, "DeQueue length is invalid!");
        assert_eq!(queue.front, 0, "DeQueue front index is invalid!");
        assert_eq!(queue.rear, 1, "DeQueue rear index is invalid!");
        assert_eq!(format!("{}", queue.data), "[1]", "DeQueue is invalid!");
        queue.enqueue_back(2).expect("Failed to enqueue!");
        queue.enqueue_back(3).expect("Failed to enqueue!");
        assert_eq!(queue.length, 3, "DeQueue length is invalid!");
        assert_eq!(queue.front, 0, "DeQueue front index is invalid!");
        assert_eq!(queue.rear, 0, "DeQueue rear index is invalid!");
        assert!(
            queue.enqueue_back(3).is_err(),
            "DeQueue must throw error for being full!"
        );
        queue.dequeue_front().expect("Failed to dequeue!");
        queue.enqueue_back(4).expect("Failed to enqueue!");
        assert_eq!(queue.length, 3, "DeQueue length is invalid!");
        assert_eq!(queue.front, 1, "DeQueue front index is invalid!");
        assert_eq!(queue.rear, 1, "DeQueue rear index is invalid!");
        assert_eq!(
            format!("{}", queue.data),
            "[4, 2, 3]",
            "DeQueue is invalid!"
        );
        assert!(
            queue.enqueue_back(3).is_err(),
            "DeQueue must throw error for being full!"
        );
    }

    // #[test]
    // fn test_dequeue() {
    //     let mut queue: DeQueue<u8> = DeQueue::new(3);
    //     queue.enqueue(1).expect("Failed to enqueue!");
    //     queue.enqueue(2).expect("Failed to enqueue!");
    //     assert_eq!(queue.dequeue().expect("Failed to dequeue!"), 1, "DeQueue dequeue returned value is invalid!");
    //     assert_eq!(queue.length, 1, "DeQueue length is invalid!");
    //     assert_eq!(queue.front, 1, "DeQueue front index is invalid!");
    //     assert_eq!(queue.rear, 2, "DeQueue rear index is invalid!");
    //     queue.enqueue(3).expect("Failed to enqueue!");
    //     queue.enqueue(4).expect("Failed to enqueue!");
    //     println!("{}", queue.data);
    //     assert_eq!(queue.front, 1, "DeQueue front index is invalid!");
    //     assert_eq!(queue.rear, 1, "DeQueue rear index is invalid!");
    //     assert_eq!(queue.dequeue().expect("Failed to dequeue!"), 2, "DeQueue dequeue returned value is invalid!");
    //     assert_eq!(queue.length, 2, "DeQueue length is invalid!");
    //     assert_eq!(queue.front, 2, "DeQueue front index is invalid!");
    //     assert_eq!(queue.rear, 1, "DeQueue rear index is invalid!");
    //     queue.enqueue(5).expect("Failed to enqueue!");
    //     queue.dequeue().expect("Failed to dequeue!");
    //     queue.dequeue().expect("Failed to dequeue!");
    //     queue.dequeue().expect("Failed to dequeue!");
    //     assert!(queue.dequeue().is_err(), "DeQueue must throw error for being full!");
    // }
    //
    // #[test]
    // fn test_is_empty() {
    //     let mut queue: DeQueue<u8> = DeQueue::new(5);
    //     assert_eq!(queue.is_empty(), true, "DeQueue must be empty!");
    //     queue.enqueue(5);
    //     assert_eq!(queue.is_empty(), false, "DeQueue must not be empty!");
    // }
    //
    // #[test]
    // fn test_is_full() {
    //     let mut queue: DeQueue<u8> = DeQueue::new(2);
    //     queue.enqueue(5).expect("Error");
    //     assert_eq!(queue.is_full(), false, "DeQueue must not be full!");
    //     queue.enqueue(5).expect("Error");
    //     assert_eq!(queue.is_full(), true, "DeQueue must be full!");
    // }
}
