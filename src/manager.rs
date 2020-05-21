use chashmap::CHashMap;

use crate::queue::TinQueue;

pub struct TinQueueManager {
    map: CHashMap<String, TinQueue<String>>
}

impl TinQueueManager {
    // Create new Tin Queue Manager
    pub fn new() -> Self {
        Self {
            map: CHashMap::new(),
        }
    }

    // Add new Tin Queue to the map
    pub fn add_queue(&self, queue_name: String, size: usize) -> bool {
        if let Some(_) = self.map.get(&queue_name) {
            return false;
        } else {
            let queue = TinQueue::new(size);
            self.map.insert_new(queue_name, queue);
            true
        }
    }

    // Get Tin Queue from the manager
    pub fn get_queue(&self, queue_name: String) -> Option<TinQueue<String>> {
        match &mut self.map.get_mut(&queue_name) {
            Some(queue) => {
                return Some(queue.to_owned());
            },
            None => {
                None
            }
        }
    }

    // Update Tin Queue
    pub fn update_queue(&self, queue_name: String, queue: TinQueue<String>) {
        self.map.insert(queue_name, queue);
    }

    // Delete a Tin Queue from the manager
    pub fn delete_queue(&self, queue_name: String) -> Option<TinQueue<String>> {
        self.map.remove(&queue_name)
    }
}