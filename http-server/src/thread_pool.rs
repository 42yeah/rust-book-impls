use std::{sync::{Arc, Mutex, mpsc::{self, Sender}}, thread::{self, JoinHandle}};

pub struct ThreadPool {
    threads: Vec<Option<JoinHandle<()>>>,
    sender: Sender<Msg>
}

pub enum Msg {
    Job(Box<dyn FnOnce() + Send>),
    Terminate
}

impl ThreadPool {
    pub fn new(num_threads: usize) -> ThreadPool {
        let mut threads = Vec::with_capacity(num_threads);
        let (tx, rx) = mpsc::channel();
        
        let receiver = Arc::new(Mutex::new(rx));

        for i in 0..num_threads {
            let receiver = receiver.clone();
            threads.push(Some(thread::spawn(move || loop {
                let job: Msg = receiver.lock().unwrap().recv().unwrap();
                match job {
                    Msg::Job(job) => {
                        println!("I am worker {} and I have received a job.", i);
                        job();
                    }
                    Msg::Terminate => {
                        println!("Time to go!");
                        break;
                    }
                }
            })));
        }
        ThreadPool {
            threads,
            sender: tx
        }
    }

    pub fn execute<T>(&self, closure: T) where T: FnOnce() + Send + 'static {
        self.sender.send(Msg::Job(Box::new(closure))).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for _ in self.threads.iter() {
            self.sender.send(Msg::Terminate).unwrap();
        }
        for thread in &mut self.threads {
            if let Some(thread) = thread.take() {
                thread.join().unwrap();
            }
        }
    }
}