use core::cell::{RefCell, RefMut};

/// 允许单核上安全使用可变全局变量
pub struct UPSafeCell<T> {
    inner: RefCell<T>,
}

/// 标记为Sync,使其作为一个全局变量，因为编译器无法确定是否被多个线程使用，所以该操作是unsafe的
unsafe impl<T> Sync for UPSafeCell<T> {}

impl<T> UPSafeCell<T> {
    ///
    pub unsafe fn new(value: T) -> Self {
        Self {
            inner: RefCell::new(value),
        }
    }

    /// 获取独占访问权，可变借用标记
    pub fn exclusive_access(&self) -> RefMut<'_, T> {
        self.inner.borrow_mut()
    }
}
