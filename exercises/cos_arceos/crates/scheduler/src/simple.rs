use alloc::{collections::VecDeque, sync::Arc};
use core::ops::{Deref, DerefMut};
use core::ptr;

use crate::BaseScheduler;

/// A task wrapper for the [`SimpleScheduler`].
pub struct SimpleTask<T> {
    inner: T,
    time_slice: usize
}

impl<T> SimpleTask<T> {
    /// Creates a new [`SimpleTask`] from the inner task struct.
    pub const fn new(inner: T) -> Self {
        Self {
            inner,
            time_slice: 10
        }
    }

    /// Returns a reference to the inner task struct.
    pub const fn inner(&self) -> &T {
        &self.inner
    }

    pub fn time_slice(&self) -> usize {
        self.time_slice
    }

    // pub fn decremented_time_slice(&mut self) {
    //     if self.time_slice > 0 {
    //         let time_slice_ptr = &self.time_slice as *const _ as *mut isize;
    //         unsafe {
    //             ptr::write(time_slice_ptr, (&self.time_slice - 1)as isize);
    //         }
    //     }
    // }
    // pub fn reset_time_slice(&self) {
    //     let time_slice_ptr = &self.time_slice as *const _ as *mut isize;
    //     unsafe {
    //         ptr::write(time_slice_ptr, 10);
    //     }
    // }
}

impl<T> Deref for SimpleTask<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for SimpleTask<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// A simple scheduler.
///
/// When a task is added to the scheduler, it's placed at the end of the ready
/// queue. When picking the next task to run, the head of the ready queue is
/// taken.
///
/// As it's a cooperative scheduler, it does nothing when the timer tick occurs.
///
pub struct SimpleScheduler<T> {
    ready_queue: VecDeque<Arc<SimpleTask<T>>>,
    // max_time_slice: usize
}

impl<T> SimpleScheduler<T> {
    /// Creates a new empty [`SimpleScheduler`].
    pub const fn new() -> Self {
        Self {
            ready_queue: VecDeque::new(),
            // max_time_slice: 10,
        }
    }
    /// get the name of scheduler
    pub fn scheduler_name() -> &'static str {
        "Simple"
    }
}

impl<T> BaseScheduler for SimpleScheduler<T> {
    type SchedItem = Arc<SimpleTask<T>>;

    fn init(&mut self) {
        
        // for task in &mut self.ready_queue {
        //     let time_slice_ptr = &task.time_slice as *const _ as *mut isize;
        //     unsafe {
        //         ptr::write(time_slice_ptr, self.max_time_slice as isize);
        //     }
        // }
    }

    fn add_task(&mut self, task: Self::SchedItem) {
        trace!("######### add_task");
        self.ready_queue.push_back(task);
    }

    fn remove_task(&mut self, task: &Self::SchedItem) -> Option<Self::SchedItem> {
        trace!("######### remove_task");
        self.ready_queue
            .iter()
            .position(|t| Arc::ptr_eq(t, task))
            .and_then(|idx| self.ready_queue.remove(idx))
    }

    fn pick_next_task(&mut self) -> Option<Self::SchedItem> {
        self.ready_queue.pop_front()
    }

    fn put_prev_task(&mut self, prev: Self::SchedItem, _preempt: bool) {
        if prev.time_slice() > 0 && _preempt {
            self.ready_queue.push_front(prev);
        } else {
            let time_slice_ptr = &prev.time_slice as *const _ as *mut isize;
            unsafe {
                ptr::write(time_slice_ptr, 10);
            }
            self.ready_queue.push_back(prev);
        }
    }

    fn task_tick(&mut self, _current: &Self::SchedItem) -> bool {
        // false // no reschedule
        let time_slice_ptr = &_current.time_slice as *const _ as *mut isize;
            unsafe {
                ptr::write(time_slice_ptr, (&_current.time_slice - 1)as isize);
            }
        _current.time_slice() == 0
    }

    fn set_priority(&mut self, _task: &Self::SchedItem, _prio: isize) -> bool {
        false
    }
}
