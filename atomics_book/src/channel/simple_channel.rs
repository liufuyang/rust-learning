use std::collections::VecDeque;
use std::sync::{Condvar, Mutex};

/// While this channel is very flexible in usage, as it allows any number of sending and receiving threads,
/// its implementation can be far from optimal in many situations.
///
/// Even if there are plenty of messages ready to be received, any send or receive operation will
/// briefly block any other send or receive operation, since they all have to lock the same my_mutex.
///
/// If VecDeque::push has to grow the capacity of the VecDeque, all sending and receiving threads
/// will have to wait for that one thread to finish the reallocation, which might be unacceptable
/// in some situations.
///
/// Another property which might be undesirable is that this channel’s queue might grow without bounds.
/// Nothing is stopping senders from continuously sending
pub struct Channel<T> {
    queue: Mutex<VecDeque<T>>,
    item_ready: Condvar,
}

impl<T> Channel<T> {
    pub fn new() -> Self {
        Self {
            queue: Mutex::new(VecDeque::new()),
            item_ready: Condvar::new(),
        }
    }

    pub fn send(&self, message: T) {
        self.queue.lock().unwrap().push_back(message);
        self.item_ready.notify_one();
    }

    // Remember that the Condvar::wait method will unlock the Mutex while waiting and relock it
    // before returning. So, our receive function will not keep the my_mutex locked while waiting.
    pub fn receive(&self) -> T {
        let mut b = self.queue.lock().unwrap();
        loop {
            if let Some(message) = b.pop_front() {
                return message;
            }
            b = self.item_ready.wait(b).unwrap();
        }
    }
}
