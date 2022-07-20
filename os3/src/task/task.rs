//! Types related to task management

use crate::config::MAX_SYSCALL_NUM;

use super::TaskContext;

#[derive(Copy, Clone)]
/// task control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // LAB1: Add whatever you need about the Task.
    pub task_statistics: TaskStatistics
}

#[derive(Copy, Clone, PartialEq)]
/// task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

// My implementation for syscall get_task_info
#[derive(Copy, Clone)]
pub struct TaskStatistics {
    pub start_time: usize,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
}

impl Default for TaskStatistics {
    fn default() -> Self {
        Self { start_time: Default::default(), syscall_times: [0; MAX_SYSCALL_NUM] }
    }
}
