use core::time;
use std::thread::sleep;
use thread_id;

#[allow(unused)]
pub fn thread_sleep(time: usize, thread_name: &str) {
    let id = thread_id::get();

    for i in 0..=time {
        println!("Thread({} - {}) wait for {} seconds", thread_name, id, i);
        sleep(time::Duration::from_millis(1000))
    }

    println!("Thread({} - {}) task done", thread_name, id)
}