use std::sync::mpsc;
use std::sync::{Arc, Mutex, Condvar};
use std::thread;
use std::time::Duration;
use std::vec::Vec;
use std::error;
use std::sync::atomic;

#[derive(Debug)]
pub struct Queue<T> {
    sender: mpsc::Sender<T>,
    receiver: mpsc::Receiver<T>,
    counter: atomic::AtomicIsize,
}

impl<T> Queue<T> {
    fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        Queue { sender: tx, receiver: rx, counter: atomic::AtomicIsize::new(0) }
    }

    fn push(&mut self, item: T) {
        self.counter.fetch_add(1, atomic::Ordering::SeqCst);
        self.sender.send(item);
    }

    fn pop(&mut self) -> Result<T, mpsc::RecvError> {
        self.counter.fetch_sub(1, atomic::Ordering::SeqCst);
        self.receiver.recv()
    }

    fn size(&mut self) -> i64 {
        let c: isize = self.counter.load(atomic::Ordering::SeqCst);
        println!("counter value type {}", &c);
        c as i64
    }
}

impl<T> Clone for Queue<T> {
    fn clone(&self) -> Queue<T> {
        Queue{
            sender: self.sender,
            receiver: self.receiver,
            counter: self.counter,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::thread;
    use super::Queue;

    #[test]
    pub fn queue_test() {
        let mut q: Queue<i32> = Queue::new();
        let mut q_clone = q.clone();

        (0..50).for_each( |x| {
            thread::spawn(move || {
                (0..50).for_each(|x| {
                    q.push(1);
                });
            });
        });

        thread::sleep_ms(3000);

        q_clone.push(1);

        println!("size: {}", q_clone.size());
    }
}

pub fn sync_test() {
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair_clone = pair.clone();
    let pair_main = pair.clone();

    thread::spawn(move || {
        let (ref mutex, ref condvar) = *pair_clone;

        let mut started = mutex.lock().unwrap();
        println!("child: try to set started value");
        thread::sleep(Duration::from_secs(5));
        *started = true;
        println!("child: set started value after 5s");
        condvar.notify_one();

        println!("child: notify main thread");
    });

    let (ref mutex, ref condvar) = *pair_main;
    let mut started = mutex.lock().unwrap();

    while !*started {
        println!("parent: before wait");
        started = condvar.wait(started).unwrap();
        println!("parent: after wait");
    }
}

