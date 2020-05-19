use std::iter::{Chain};
use std::slice::{Iter as SliceIter, IterMut as SliceIterMut};

pub struct Queue<T> {
    vec: Vec<T>,
    capacity: usize,
    index: usize
}

pub type Iter<'a, T> = Chain<SliceIter<'a, T>, SliceIter<'a, T>>;
pub type IterMut<'a, T> = Chain<SliceIterMut<'a, T>, SliceIterMut<'a, T>>;

impl<T> Queue<T> {
    // Creates new Queue
    pub fn new(size: usize) -> Self {
        Self {
            vec: Vec::with_capacity(size),
            capacity: size,
            index: 0
        }
    }

    // Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    // Returns the number of elements in the queue
    pub fn get_len(&self) -> usize {
        self.vec.len()
    }

    // Returns the queue capacity
    pub fn get_capacity(&self) -> usize {
        self.capacity
    }

    // Clear the queue
    pub fn clear_queue(&mut self) {
        self.vec.clear();
        self.index = 0;
    }

    // Push item into the queue
    pub fn push(&mut self, element: T) {
        // Return if no capacity
        if self.capacity == 0 {
            return;
        }
        if self.vec.len() < self.capacity {
            self.vec.push(element);
        } else {
            self.vec[self.index] = element;
        }
        self.index = (self.index + 1) % self.get_capacity();
    }

    // Iter through the queue content and return the iterator
    pub fn iter(&self) -> Iter<T> {
        let (a, b) = self.vec.split_at(self.index);
        b.iter().chain(a.iter())
    }

    // Iter through the queue content and return the mutable iterator
    pub fn iter_mut(&mut self) -> IterMut<T> {
        let (a, b) = self.vec.split_at_mut(self.index);
        b.iter_mut().chain(a.iter_mut())
    }
}