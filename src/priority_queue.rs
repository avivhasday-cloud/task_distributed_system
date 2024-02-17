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


#[cfg(test)]
mod tests {

    use super::*;


    struct TestContext {
        queue: PriorityQueue,
    }
    impl Drop for TestContext {
        fn drop(&mut self) {
            println!("Test teardown ...");
        }
    }

    fn setup() -> TestContext {

        TestContext {
            queue: PriorityQueue::new()
        }
    }

    #[test]
    fn test_enqueue() {
        let mut ctx = setup();
        let task = Task::new("test-user","test task", "test description", "High");
        ctx.queue.enqueue(task);
        assert_eq!(ctx.queue.data.is_empty(), false);
    }

    #[test]
    fn test_is_empty() {
        let mut ctx = setup();
        assert_eq!(ctx.queue.is_empty(), true);        
    
        let task = Task::new("test-user","test task", "test description", "High");
        ctx.queue.enqueue(task);
        assert_eq!(ctx.queue.is_empty(), false);
    }

    #[test]
    fn test_len() {
        let mut ctx = setup();
        assert_eq!(ctx.queue.len(), 0);        
    
        let task = Task::new("test-user","test task", "test description", "High");
        ctx.queue.enqueue(task);
        assert_eq!(ctx.queue.len(), 1);
    }

    #[test]
    fn test_get() {
        let mut ctx = setup();    
        let task = Task::new("test-user","test task", "test description", "High");
        ctx.queue.enqueue(task.clone());

        let task_from_queue = ctx.queue.get().unwrap().clone();
        assert_eq!(&task_from_queue, &task);
    }


    #[test]
    fn test_get_queued_items() {
        let mut ctx = setup();    
        let task = Task::new("test-user","test task", "test description", "High");
        ctx.queue.enqueue(task.clone());
        
        let queued_items = ctx.queue.get_queued_items();
        assert_eq!(queued_items.len(), 1);
    }

}