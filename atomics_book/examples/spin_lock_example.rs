use atomics_book::spin_lock::r#unsafe::SpinLockUnsafe;
use atomics_book::spin_lock::safe::SpinLock;
use std::ops::{Deref, DerefMut};
use std::thread;

fn main() {
    let lock_unsafe = SpinLockUnsafe::new(0);
    let lock = SpinLock::new(0);
    let x = SpinLock::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| {
            let v = lock_unsafe.lock();
            *v = *v + 1;
            println!("A - lock_unsafe - v:{}", *v);
            unsafe { lock_unsafe.unlock() }
            *v = *v + 1; // Very unsafe as other thread will either read before or after this happens.

            // safe lock usage
            let mut v = lock.lock();
            *v.deref_mut() = v.deref() + 1;
            println!("A - lock - v:{}", *v);

            // another safe lock usage
            x.lock().push(1)
        });

        s.spawn(|| {
            let v = lock_unsafe.lock();
            *v = *v + 1;
            println!("B - lock_unsafe - v:{}", *v); // can be 2 or 3.
            unsafe { lock_unsafe.unlock() }

            // safe lock usage
            let mut v = lock.lock();
            *v.deref_mut() = v.deref() + 1;
            println!("B - lock - v:{}", *v);

            // another safe lock usage
            let mut g = x.lock();
            g.push(2);
            g.push(2);
        });
    });

    let g = x.lock();
    assert!(g.as_slice() == [1, 2, 2] || g.as_slice() == [2, 2, 1]);
}
