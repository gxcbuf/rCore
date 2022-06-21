use super::TaskContext;

#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    pub status: TaskStatus,
    pub ctx: TaskContext,
}

#[derive(Clone, Copy, PartialEq)]
pub enum TaskStatus {
    UnInit,  // 未初始化
    Ready,   // 准备运行
    Running, // 正在运行
    Exited,  // 已退出
}
