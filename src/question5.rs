// 使用多线程并行计算某个函数的值或模拟并发任务。
// 需要创建 3 个线程同时进行下载，并在下载完成后将结果（例如“URL + 下载完成”）
// 通过消息通道（std::sync::mpsc）发送回主线程。主线程依次接收并打印结果。
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

// 模拟下载任务
fn download(url: &str, sender: mpsc::Sender<String>) {
    println!("开始下载: {}", url);
    thread::sleep(Duration::from_secs(2)); // 模拟下载时间
    let result = format!("{} 下载完成", url);
    sender.send(result).unwrap(); // 发送消息给主线程
}

pub fn parallel_download() {
    let urls = vec![
        "https://example.com/file1",
        "https://example.com/file2",
        "https://example.com/file3",
    ];
    
    let (tx, rx) = mpsc::channel(); // 创建消息通道
    
    let mut handles = vec![];
    
    for url in urls {
        let tx_clone = tx.clone(); // 克隆发送端
        let handle = thread::spawn(move || {
            download(url, tx_clone);
        });
        handles.push(handle);
    }
    
    drop(tx); // 关闭原始发送端，确保接收端不会无限等待
    
    // 依次接收并打印结果
    for received in rx {
        println!("{}", received);
    }
    
    // 等待所有线程完成
    for handle in handles {
        handle.join().unwrap();
    }
}