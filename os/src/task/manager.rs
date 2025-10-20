//!Implementation of [`TaskManager`]
use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::BinaryHeap;
use alloc::sync::Arc;
use core::cmp::Ordering;
use lazy_static::*;


struct HeapEntry {
    stride: super::task::Stride,
    task: Arc<TaskControlBlock>,
}

impl PartialEq for HeapEntry {
    fn eq(&self, other: &Self) -> bool {
        self.stride == other.stride && Arc::ptr_eq(&self.task, &other.task)
    }
}

impl Eq for HeapEntry {}

impl PartialOrd for HeapEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        match other.stride.cmp(&self.stride) {
            Ordering::Equal => {
                let self_ptr = Arc::as_ptr(&self.task);
                let other_ptr = Arc::as_ptr(&other.task);
                other_ptr.cmp(&self_ptr)
            }
            ordering => ordering,
        }
    }
}

///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    heap: BinaryHeap<HeapEntry>,
}

/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        let stride = {
            let inner = task.inner_exclusive_access();
            inner.stride
        };
        self.heap.push(HeapEntry { stride, task });
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        self.heap.pop().map(|entry| entry.task)
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    trace!("kernel: TaskManager::add_task");
    TASK_MANAGER.exclusive_access().add(task);
}

/// Take a process out of the ready queue
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    trace!("kernel: TaskManager::fetch_task");
    let task = TASK_MANAGER.exclusive_access().fetch()?;
    let inner = task.inner_exclusive_access();
    drop(inner);

    Some(task)
}
