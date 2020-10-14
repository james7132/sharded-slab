pub(crate) use self::inner::*;

#[cfg(test)]
mod inner {
    pub(crate) use loom::cell::UnsafeCell;
    pub(crate) mod atomic {
        pub use loom::sync::atomic::*;
        pub use std::sync::atomic::Ordering;
    }
    pub(crate) use loom::lazy_static;
    pub(crate) use loom::sync::Mutex;
    pub(crate) use loom::thread::yield_now;
    pub(crate) use loom::thread_local;
}

#[cfg(not(test))]
mod inner {
    pub(crate) use lazy_static::lazy_static;
    pub(crate) use std::sync::atomic;
    pub(crate) use std::sync::Mutex;
    pub(crate) use std::thread::yield_now;
    pub(crate) use std::thread_local;

    #[derive(Debug)]
    pub struct UnsafeCell<T>(std::cell::UnsafeCell<T>);

    impl<T> UnsafeCell<T> {
        pub fn new(data: T) -> UnsafeCell<T> {
            UnsafeCell(std::cell::UnsafeCell::new(data))
        }

        #[inline(always)]
        pub fn with<F, R>(&self, f: F) -> R
        where
            F: FnOnce(*const T) -> R,
        {
            f(self.0.get())
        }

        #[inline(always)]
        pub fn with_mut<F, R>(&self, f: F) -> R
        where
            F: FnOnce(*mut T) -> R,
        {
            f(self.0.get())
        }
    }
}
