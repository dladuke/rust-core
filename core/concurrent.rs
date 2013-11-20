// Copyright 2013 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Concurrent data structures

use super::clone::Clone;
use super::arc::Arc;
use super::deque::Deque;
use super::mem::transmute;
use super::thread::{Mutex, Cond};

#[no_freeze]
struct QueueBox<T> {
    deque: Deque<T>,
    mutex: Mutex,
    not_empty: Cond
}

/// An unbounded, blocking concurrent queue
pub struct Queue<T> {
    priv ptr: Arc<QueueBox<T>>
}

impl<T> Queue<T> {
    /// Return a new `Queue` instance
    pub fn new() -> Queue<T> {
        unsafe {
            let box = QueueBox { deque: Deque::new(), mutex: Mutex::new(), not_empty: Cond::new() };
            Queue { ptr: Arc::new_unchecked(box) }
        }
    }

    /// Pop a value from the front of the queue, blocking until the queue is not empty
    pub fn pop(&self) -> T {
        unsafe {
            let box: &mut QueueBox<T> = transmute(self.ptr.borrow());
            let mut guard = box.mutex.lock_guard();
            while box.deque.len() == 0 {
                box.not_empty.wait_guard(&mut guard)
            }
            box.deque.pop_front().get()
        }
    }

    /// Push a value to the back of the queue
    pub fn push(&self, item: T) {
        unsafe {
            let box: &mut QueueBox<T> = transmute(self.ptr.borrow());
            box.mutex.lock();
            box.deque.push_back(item);
            box.mutex.unlock();
            box.not_empty.signal()
        }
    }
}

impl<T> Clone for Queue<T> {
    /// Return a shallow copy of the `Queue`
    fn clone(&self) -> Queue<T> {
        Queue { ptr: self.ptr.clone() }
    }
}

#[no_freeze]
struct BoundedQueueBox<T> {
    deque: Deque<T>,
    mutex: Mutex,
    not_empty: Cond,
    not_full: Cond,
    maximum: uint
}

/// A bounded, blocking concurrent queue
pub struct BoundedQueue<T> {
    priv ptr: Arc<BoundedQueueBox<T>>
}

impl<T> BoundedQueue<T> {
    /// Return a new `BoundedQueue` instance, holding at most `maximum` elements
    pub fn new(maximum: uint) -> BoundedQueue<T> {
        unsafe {
            let box = BoundedQueueBox { deque: Deque::new(), mutex: Mutex::new(), not_empty: Cond::new(),
                                        not_full: Cond::new(), maximum: maximum };
            BoundedQueue { ptr: Arc::new_unchecked(box) }
        }
    }

    /// Pop a value from the front of the queue, blocking until the queue is not empty
    pub fn pop(&self) -> T {
        unsafe {
            let box: &mut BoundedQueueBox<T> = transmute(self.ptr.borrow());
            box.mutex.lock();
            while box.deque.len() == 0 {
                box.not_empty.wait(&mut box.mutex)
            }
            let item = box.deque.pop_front().get();
            box.mutex.unlock();
            box.not_full.signal();
            item
        }
    }

    /// Push a value to the back of the queue, blocking until the queue is not full
    pub fn push(&self, item: T) {
        unsafe {
            let box: &mut BoundedQueueBox<T> = transmute(self.ptr.borrow());
            box.mutex.lock();
            while box.deque.len() == box.maximum {
                box.not_full.wait(&mut box.mutex)
            }
            box.deque.push_back(item);
            box.mutex.unlock();
            box.not_empty.signal()
        }
    }
}

impl<T> Clone for BoundedQueue<T> {
    /// Return a shallow copy of the `BlockingQueue`
    fn clone(&self) -> BoundedQueue<T> {
        BoundedQueue { ptr: self.ptr.clone() }
    }
}