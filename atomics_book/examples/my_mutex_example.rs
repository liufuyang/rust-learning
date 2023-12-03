use atomics_book::my_mutex::MyMutex;
use std::ops::{Deref, DerefMut};
use std::thread;

fn main() {
    let lock = MyMutex::new(0);
    let x = MyMutex::new(Vec::new());
    thread::scope(|s| {
        s.spawn(|| {
            // safe lock usage
            let mut v = lock.lock();
            *v.deref_mut() = v.deref() + 1;
            println!("A - lock - v:{}", *v);

            // another safe lock usage
            x.lock().push(1)
        });

        s.spawn(|| {
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
