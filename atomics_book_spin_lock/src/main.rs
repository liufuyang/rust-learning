use std::ops::{Deref, DerefMut};
use std::thread;
use atomics_book_spin_lock::spin_lock::r#unsafe::SpinLockUnsafe;
use lazy_static::lazy_static;
use atomics_book_spin_lock::spin_lock::safe::SpinLock;

lazy_static! {
    static ref LOCK_UNSAFE: SpinLockUnsafe<i32> = {
        SpinLockUnsafe::new(0)
    };

    static ref LOCK: SpinLock<i32> = {
        SpinLock::new(0)
    };
}

fn main() {
    println!("Hello, world!");

    let t1 = thread::spawn(move || {
        let v = LOCK_UNSAFE.lock();
        *v = *v + 1;
        println!("LOCK_UNSAFE - v:{}", *v);
        unsafe {LOCK_UNSAFE.unlock()}
        *v = *v + 1; // Very unsafe as other thread will either read before or after this happens.

        // safe lock usage
        let mut v = LOCK.lock();
        *v.deref_mut() = v.deref() + 1;
        println!("lock - v:{}", *v);
    });
    let t2 = thread::spawn(move || {
        let v = LOCK_UNSAFE.lock();
        *v = *v + 1;
        println!("l1 - v:{}", *v);
        unsafe {LOCK_UNSAFE.unlock()}

        // safe lock usage
        let mut v = LOCK.lock();
        *v.deref_mut() = v.deref() + 1;
        println!("lock - v:{}", *v);
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
