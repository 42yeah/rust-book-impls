use std::{sync::{Arc, Mutex}, thread::{self, JoinHandle}, time::Duration};

fn main() {
    let mutex = Arc::new(Mutex::new(1));
    let mut threads: Vec<JoinHandle<()>> = vec![];
    
    for i in 0..10 {
        let mutex = mutex.clone();
        threads.push(thread::spawn(move || {
            let mut value = mutex.lock().unwrap();
            println!("Thread {} got: {}. Adding it by one.", i, *value);
            *value += 1;
        }));
    }
    
    for thread in threads {
        thread.join().unwrap();
    }

    // Now for some (potential) deadlock!
    let mutex_one = Arc::new(Mutex::new(42));
    let mutex_two = Arc::new(Mutex::new(1));
    let mut threads: Vec<JoinHandle<()>> = vec![];

    for i in 0..2 {
        let m_one = mutex_one.clone();
        let m_two = mutex_two.clone();
        threads.push(thread::spawn(move || {
            match i {
                0 => {
                    let one_val = m_one.lock().unwrap();
                    thread::sleep(Duration::from_millis(100));
                    let mut two_val = m_two.lock().unwrap();
                    *two_val += *one_val;
                },
                1 => {
                    let two_val = m_two.lock().unwrap();
                    let mut one_val = m_one.lock().unwrap();
                    *one_val += *two_val;
                },
                _ => panic!("Unreachable")
            };
        }));
    }

    for thread in threads {
        thread.join().unwrap();
    }
    println!("{} {}", mutex_one.lock().unwrap(), mutex_two.lock().unwrap());
}
