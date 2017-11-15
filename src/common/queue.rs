//use std::sync::mpsc;
//use std::sync::{Arc, Mutex, Condvar};
//use std::thread;
//use std::time::Duration;
//use std::vec::Vec;
//use std::error;
//use std::sync::atomic;

//#[derive(Debug)]
//pub struct Queue<'a, T: 'a> {
//    sender: &'a mpsc::Sender<T>,
//    receiver: &'a mpsc::Receiver<T>,
//    counter: &'a atomic::AtomicIsize,
//}
//
//impl<'a, T> Queue<'a, T> {
//    fn new() -> Self {
//        let (tx, rx) = mpsc::channel();
//        Queue { sender: &tx, receiver: &rx, counter: &atomic::AtomicIsize::new(0) }
//    }
//
//    fn push(&mut self, item: T) {
//        self.counter.fetch_add(1, atomic::Ordering::SeqCst);
//        self.sender.send(item);
//    }
//
//    fn pop(&mut self) -> Result<T, mpsc::RecvError> {
//        self.counter.fetch_sub(1, atomic::Ordering::SeqCst);
//        self.receiver.recv()
//    }
//
//    fn size(&mut self) -> i64 {
//        let c: isize = self.counter.load(atomic::Ordering::SeqCst);
//        println!("counter value type {}", &c);
//        c as i64
//    }
//}
//
//impl<'a, T> Clone for Queue<'a, T> {
//    fn clone(&self) -> Queue<'a, T> {
//        Queue{
//            sender: &self.sender.clone(),
//            receiver: self.receiver,
//            counter: self.counter,
//        }
//    }
//}

#[cfg(test)]
mod tests {
    use std::sync::mpsc;
    use std::sync::{Arc, Mutex, Condvar};
    use std::thread;
    use std::time::Duration;
//    use super::Queue;

    #[test]
//    pub fn queue_test() {
//        let q: Queue<i32> = Queue::new();
////        let mut arc_queue = Arc::new(q);
//        let mut q_clone = q.clone();
//
////        (0..50).for_each(|_| {
//            thread::spawn( || {
//                (0..50).for_each(|x| {
//                    q_clone.push(1);
//                });
//            });
////        });
//
////        thread::sleep_ms(3000);
////
////        q_clone.push(1);
////
////        println!("size: {}", q_clone.size());
//    }

    #[test]
    pub fn sync_test() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair_child = pair.clone();
        let pair_main = pair.clone();

        thread::spawn(move || {
            let (ref mutex, ref condvar) = *pair_child;

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

    #[test]
    pub fn channel_test() {
        println!("-");

        let (tx, rx) = mpsc::channel();

        let t1 = thread::spawn(move || {
            thread::sleep(Duration::from_secs(5));

            let v = 10;
            tx.send(v).unwrap();
            println!("write value {}", v);
        });

        let t3 = thread::spawn(move || {
            let v = rx.recv().unwrap();
            println!("read value {}", v);
        });

        t1.join().unwrap();
        t3.join().unwrap();
    }
}