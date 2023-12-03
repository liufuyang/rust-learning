use std::cell::UnsafeCell;
use std::ops::{Deref, DerefMut};
use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::{Acquire, Release};
use atomic_wait::wait;
use atomic_wait::wake_one;

/// To be able to provide a fully safe interface, we need to tie the unlocking operation to
/// the end of the &mut T. We can do that by wrapping this reference in our own type that
/// behaves like a reference, but also implements the Drop trait to do something when it is dropped.
///
/// Such a type is often called a guard, as it effectively guards the state of the lock,
/// and stays responsible for that state until it is dropped.
pub struct Guard<'a, T> {
    lock: &'a MyMutex<T>,
}

pub struct MyMutex<T> {
    /// 0: unlocked
    /// 1: locked
    state: AtomicU32,
    value: UnsafeCell<T>,
}

unsafe impl<T> Sync for MyMutex<T> where T: Send {}

impl<T> MyMutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            state: AtomicU32::new(0),
            value: UnsafeCell::new(value),
        }
    }

    pub fn lock(&self) -> Guard<T> {
        // swap: Stores a value into the atomic integer, returning the previous value.
        // Set the state to 1: locked.
        while self.state.swap(1, Acquire) == 1 {
            // If getting here, means the self.state was storing 1 (locked) before swap.
            // So we wait.
            // And using atomic_wait::wait so to also make sure in case the state is updated again.
            // A while is used as wait might also return spuriously, without a corresponding wake operation.
            wait(&self.state, 1);
        }
        Guard { lock: self }
    }
}

impl<T> Drop for Guard<'_, T> {
    fn drop(&mut self) {
        // Set the state back to 0: unlocked.
        self.lock.state.store(0, Release);
        // Wake up one of the waiting threads, if any.
        wake_one(&self.lock.state);
    }
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Safety: The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &mut *self.lock.value.get() }
    }
}
