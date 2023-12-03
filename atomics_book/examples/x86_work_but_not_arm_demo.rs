use std::sync::atomic::{AtomicBool, AtomicUsize, compiler_fence};
use std::sync::atomic::Ordering::{Acquire, Release, Relaxed};
use std::thread;
use std::time::Instant;
// use atomics_book::my_mutex::MyMutex;
// use atomics_book::spin_lock::safe::SpinLock as MyMutex;

fn main() {
    let start = Instant::now();

    let locked = AtomicBool::new(false);
    // let locked = MyMutex::new(0);
    let counter = AtomicUsize::new(0);

    thread::scope(|s| {
      for _ in 0..4 {
          s.spawn(|| for _ in 0..1_000_000 {
              // Acquire the lock, using the wrong memory ordering.
              while locked.swap(true, Relaxed) {}
              // let g = locked.lock();

              compiler_fence(Acquire);

              // Non-atomically increment the counter, while holding the lock.
              let old = counter.load(Relaxed);
              let new = old + 1;
              counter.store(new, Relaxed);

              // Release the lock, using the wrong memory ordering.
              compiler_fence(Release);
              locked.store(false, Relaxed)
              // drop(g)
          });
      }
    });

    println!("Time usage: {:?} - counter value: {}", start.elapsed(), counter.into_inner());
}