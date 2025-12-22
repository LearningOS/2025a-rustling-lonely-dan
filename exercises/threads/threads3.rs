use std::sync::mpsc;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Queue {
    length: u32,
    first_half: Vec<u32>,
    second_half: Vec<u32>,
}

impl Queue {
    fn new() -> Self {
        Queue {
            length: 10,
            first_half: vec![1, 2, 3, 4, 5],
            second_half: vec![6, 7, 8, 9, 10],
        }
    }
}

// 函数返回线程句柄，方便主线程等待
fn send_tx(q: Queue, tx: mpsc::Sender<u32>) -> Vec<thread::JoinHandle<()>> {
    let qc = Arc::new(q);
    let qc1 = Arc::clone(&qc);
    let qc2 = Arc::clone(&qc);
    // 克隆发送端，给第一个线程使用
    let tx1 = tx.clone();
    // 克隆发送端，给第二个线程使用
    let tx2 = tx;

    let handle1 = thread::spawn(move || {
        for val in &qc1.first_half {
            println!("sending {:?}", val);
            tx1.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    let handle2 = thread::spawn(move || {
        for val in &qc2.second_half {
            println!("sending {:?}", val);
            tx2.send(*val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    vec![handle1, handle2]
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let queue = Queue::new();
    let queue_length = queue.length;

    // 启动发送线程并获取句柄
    let handles = send_tx(queue, tx);

    let mut total_received: u32 = 0;
    // 迭代接收消息，通道关闭后会自动退出循环
    for received in rx {
        println!("Got: {}", received);
        total_received += 1;
        // 可选：收到指定数量的消息后主动退出（防止通道未关闭的情况）
        if total_received == queue_length {
            break;
        }
    }

    // 等待所有发送线程完成（确保消息都发送完毕）
    for handle in handles {
        handle.join().unwrap();
    }

    println!("total numbers received: {}", total_received);
    assert_eq!(total_received, queue_length);
}