use std::iter::{Chain};
use std::slice::{Iter as SliceIter, IterMut as SliceIterMut};

#[derive(Clone)]
pub struct TinQueue<T> {
    vec: Vec<T>,
    capacity: usize,
    index: usize,
}

pub type TinIter<'a, T> = Chain<SliceIter<'a, T>, SliceIter<'a, T>>;
pub type TinIterMut<'a, T> = Chain<SliceIterMut<'a, T>, SliceIterMut<'a, T>>;

impl<T> TinQueue<T> {
    // Creates new Queue
    pub fn new(size: usize) -> Self {
        Self {
            vec: Vec::with_capacity(size),
            capacity: size,
            index: 0,
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
        // If the length is lesser than the capacity, push the element
        if self.vec.len() < self.capacity {
            self.vec.push(element);
        // Else override an existing element
        } else {
            self.vec[self.index] = element;
        }
        // Update the index
        self.index = (self.index + 1) % self.get_capacity();
    }

    // Pop an item from the queue
    // TODO must implement a correct way to pop an item.
    pub fn pop(&mut self) -> Option<T> {
        None
    }

    // Peek an item from the queue
    pub fn peek(&self) -> Option<&T> {
        self.vec.first()
    }

    // Iter through the queue content and return the iterator
    pub fn iter(&self) -> TinIter<T> {
        let (a, b) = self.vec.split_at(self.index);
        b.iter().chain(a.iter())
    }

    // Iter through the queue content and return the mutable iterator
    pub fn iter_mut(&mut self) -> TinIterMut<T> {
        let (a, b) = self.vec.split_at_mut(self.index);
        b.iter_mut().chain(a.iter_mut())
    }
}