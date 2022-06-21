mod context;
mod switch;
mod task;

use crate::config::MAX_APP_NUM;
use crate::loader::{get_num_app, init_app_context};
use crate::sync::UPSafeCell;
use lazy_static::*;
use switch::__switch;
use task::{TaskControlBlock, TaskStatus};

pub use context::TaskContext;

pub struct TaskManager {
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

struct TaskManagerInner {
    // 任务列表
    tasks: [TaskControlBlock; MAX_APP_NUM],
    // 当前任务索引
    current_task: usize,
}

lazy_static! {
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock {
            status: TaskStatus::UnInit,
            ctx: TaskContext::zero_init(),
        }; MAX_APP_NUM];
        for (i, task) in tasks.iter_mut().enumerate() {
            task.ctx = TaskContext::goto_restore(init_app_context(i));
            task.status = TaskStatus::Ready;
        }
        TaskManager {
            num_app,
            inner: unsafe {
                UPSafeCell::new(TaskManagerInner {
                    tasks,
                    current_task: 0,
                })
            },
        }
    };
}

impl TaskManager {
    fn run_first_task(&self) -> ! {
        let mut inner = self.inner.exclusive_access();
        let task0 = &mut inner.tasks[0];
        task0.status = TaskStatus::Running;
        let next_task_ctx_ptr = &task0.ctx as *const TaskContext;
        drop(inner);
        let mut _unused = TaskContext::zero_init();
        unsafe {
            __switch(&mut _unused as *mut TaskContext, next_task_ctx_ptr);
        }
        panic!("[kernel] unreachable in run_first_task");
    }

    fn mark_current_suspended(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].status = TaskStatus::Ready;
    }

    fn mark_current_exited(&self) {
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].status = TaskStatus::Exited;
    }

    fn find_next_task(&self) -> Option<usize> {
        let inner = self.inner.exclusive_access();
        let current = inner.current_task;
        (current + 1..current + self.num_app + 1)
            .map(|id| id % self.num_app)
            .find(|id| inner.tasks[*id].status == TaskStatus::Ready)
    }

    fn run_next_task(&self) {
        if let Some(next) = self.find_next_task() {
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            inner.tasks[next].status == TaskStatus::Running;
            inner.current_task = next;
            let current_task_ctx_ptr = &mut inner.tasks[current].ctx as *mut TaskContext;
            let next_task_ctx_ptr = &inner.tasks[next].ctx as *const TaskContext;
            drop(inner);
            unsafe {
                __switch(current_task_ctx_ptr, next_task_ctx_ptr);
            }
        } else {
            panic!("[kernel] all applications completed!");
        }
    }
}

fn run_next_task() {
    TASK_MANAGER.run_next_task();
}

fn mark_current_suspended() {
    TASK_MANAGER.mark_current_suspended();
}

fn mark_current_exited() {
    TASK_MANAGER.mark_current_exited();
}

pub fn run_first_task() {
    TASK_MANAGER.run_first_task();
}

pub fn suspend_current_and_run_next() {
    mark_current_suspended();
    run_next_task();
}

pub fn exit_current_and_run_nextt() {
    mark_current_exited();
    run_next_task();
}
