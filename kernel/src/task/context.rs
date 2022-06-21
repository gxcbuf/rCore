#[repr(C)]
#[derive(Clone, Copy)]
pub struct TaskContext {
    ra: usize,      // __switch 之后返回的地址
    sp: usize,      // app的内核栈指针
    s: [usize; 12], // 用于保存s0 ~ s11寄存器
}

impl TaskContext {
    pub fn zero_init() -> Self {
        Self {
            ra: 0,
            sp: 0,
            s: [0; 12],
        }
    }

    pub fn goto_restore(kstack_ptr: usize) -> Self {
        extern "C" {
            fn __restore();
        }
        Self {
            ra: __restore as usize,
            sp: kstack_ptr,
            s: [0; 12],
        }
    }
}
