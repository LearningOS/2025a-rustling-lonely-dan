use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    // 将 JobStatus 包裹在 Mutex 中，再用 Arc 共享
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let mut handles = vec![];

    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));
            // 获取互斥锁并修改共享变量
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed += 1;
        });
        handles.push(handle);
    }

    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }

    // 所有线程完成后，获取最终的 jobs_completed 值并输出
    let final_status = status.lock().unwrap();
    println!("jobs completed: {}", final_status.jobs_completed);
}