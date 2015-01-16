#![feature(unsafe_destructor)]
#![allow(unstable)]

use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};

pub struct RMutex<T> {
    mutex: RMutexImpl,
    cell: UnsafeCell<T>,
}

unsafe impl<T: Send> Send for RMutex<T> { }
unsafe impl<T> Sync for RMutex<T> { }

// guard isn't used because it just needs to exist.
// its dropping releases the mutex.
#[allow(dead_code)]
pub struct RMutexLock<'a, T: 'a> {
    guard: RMutexImplLock<'a>,
    mutex: &'a RMutex<T>,
}

impl<T: Send> RMutex<T> {
    pub fn new(t: T) -> RMutex<T> {
        RMutex {
            mutex: RMutexImpl::alloc(),
            cell: UnsafeCell::new(t),
        }
    }

    pub fn lock<'a>(&'a self) -> RMutexLock<'a, T> {
        RMutexLock {
            guard: self.mutex.acquire(),
            mutex: self,
        }
    }
}

impl<'a, T: Send> Deref for RMutexLock<'a, T> {
    type Target = T;
    fn deref(&self) -> &T {
        unsafe { &*self.mutex.cell.get() }
    }
}

impl<'a, T: Send> DerefMut for RMutexLock<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        unsafe { &mut *self.mutex.cell.get() }
    }
}

struct RMutexImpl(*mut ());

impl RMutexImpl {
    fn alloc() -> RMutexImpl {
        use std::ptr::PtrExt;

        let c_impl = unsafe { ffi::rs_mutex_alloc() };
        if c_impl.is_null() {
            panic!("Failed to initialize RMutex!");
        }

        RMutexImpl(c_impl)
    }

    fn acquire<'a>(&'a self) -> RMutexImplLock<'a> {
        unsafe {
            let RMutexImpl(c_impl) = *self;
            ffi::rs_mutex_acquire(c_impl);
        };
        RMutexImplLock(self)
    }
}

impl Drop for RMutexImpl {
    fn drop(&mut self) {
        unsafe {
            let RMutexImpl(c_impl) = *self;
            ffi::rs_mutex_free(c_impl);
        }
    }
}

struct RMutexImplLock<'a>(&'a RMutexImpl);

#[unsafe_destructor]
impl<'a> Drop for RMutexImplLock<'a> {
    fn drop(&mut self) {
        unsafe {
            let RMutexImplLock(&RMutexImpl(c_impl)) = *self;
            ffi::rs_mutex_release(c_impl);
        }
    }
}

mod ffi {
    extern "C" {
        pub fn rs_mutex_alloc() -> *mut ();
        pub fn rs_mutex_acquire(rmtx: *mut ());
        pub fn rs_mutex_release(rmtx: *mut ());
        pub fn rs_mutex_free(rmtx: *mut ());
    }
}
