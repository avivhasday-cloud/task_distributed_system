use std::collections::BinaryHeap;

use crate::task::Task;



pub struct PriorityQueue {
    data: BinaryHeap<Task>,
}

impl PriorityQueue {
    
    pub fn new() -> Self {
        PriorityQueue {
            data: BinaryHeap::new()
        }
    }

    pub fn enqueue(&mut self, item: Task) {
        self.data.push(item);
    }

    pub fn is_empty(&self) -> bool {
        return self.data.is_empty();
    }

    pub fn len(&self) -> usize {
        return self.data.len();
    }

    pub fn get(&mut self) -> Option<Task> {
        return self.data.pop();
    }

    pub fn get_queued_items(&self) -> std::collections::binary_heap::Iter<Task> {
        return self.data.iter()
    }
}