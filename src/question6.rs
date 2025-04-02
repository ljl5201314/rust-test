// 实现一个命令行工具，对指定目录下的所有文本文件进行搜索，将匹配结果汇总后输出。
// 为增强可玩性和综合性，该工具需要支持：
// - 命令行参数（接收搜索关键词、目录路径、是否忽略大小写等）。
// - 并发搜索。
// - 消息通信。
// - 数据结构。
// - 错误处理。
// - 文件操作。
// - 迭代器与泛型（文本行迭代、搜索函数可考虑使用泛型或 trait 做一定延伸）。
// - 可选扩展：忽略大小写、正则匹配、统计行数或文件数等。

// 导入需要的库
use std::env;
use std::fs;
use std::io::{self, BufRead};
use std::sync::mpsc;
use std::thread;


// 定义搜索任务结构体
pub struct SearchTask {
    pub keyword: String,
    pub path: String,
    pub case_sensitive: bool,
}

impl SearchTask {
    fn new(keyword: String, path: String, case_sensitive: bool) -> Self {
        SearchTask {
            keyword,
            path,
            case_sensitive,
        }
    }

    // 处理搜索逻辑
    fn search_files(&self) -> Vec<String> {
        let mut results = Vec::new();
        // 遍历目录中的文件
        for entry in fs::read_dir(&self.path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            // 判断是否为文本文件
            if path.extension().map(|e| e == "txt").unwrap_or(false) {
                // 搜索文件中的内容
                let file = fs::File::open(path.clone()).unwrap();
                let reader = io::BufReader::new(file);

                for (line_number, line) in reader.lines().enumerate() {
                    let line = line.unwrap();
                    let matched = if self.case_sensitive {
                        line.contains(&self.keyword)
                    } else {
                        line.to_lowercase().contains(&self.keyword.to_lowercase())
                    };
                    
                    if matched {
                        results.push(format!("匹配: {} (行: {})", path.display(), line_number + 1));
                    }
                }
            }
        }
        results
    }
}

// 多线程并发搜索函数
pub fn search_in_directory(keyword: String, path: String, case_sensitive: bool) {
    let search_task = SearchTask::new(keyword, path, case_sensitive);
    let results = search_task.search_files();

    for result in results {
        println!("{}", result);
    }
}

pub fn run() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    let keyword = args.get(1).unwrap_or_else(|| {
        eprintln!("请提供搜索关键词!");
        std::process::exit(1);
    });
    let path = args.get(2).unwrap_or_else(|| {
        eprintln!("请提供目录路径!");
        std::process::exit(1);
    });
    let case_sensitive = args.get(3).map_or(false, |val| val == "true");
    
    // 获取目录中的所有文本文件
    let entries: Vec<_> = fs::read_dir(&path)
        .unwrap()
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .path()
                .extension()
                .map(|e| e == "txt")
                .unwrap_or(false)
        })
        .collect();

    // 使用多线程并发搜索
    let (tx, rx) = mpsc::channel();
    for entry in entries {
        let tx_clone = tx.clone();
        let keyword_clone = keyword.clone();
        let case_sensitive_clone = case_sensitive;

        thread::spawn(move || {
            let file_path = entry.path();
            if let Ok(file) = fs::File::open(file_path.clone()) {
                let reader = io::BufReader::new(file);

                for (line_number, line) in reader.lines().enumerate() {
                    if let Ok(line_content) = line {
                        let matched = if case_sensitive_clone {
                            line_content.contains(&keyword_clone)
                        } else {
                            line_content.to_lowercase().contains(&keyword_clone.to_lowercase())
                        };

                        if matched {
                            let result = format!(
                                "匹配: {} (行: {})",
                                file_path.display(),
                                line_number + 1
                            );
                            tx_clone.send(result).unwrap();
                        }
                    }
                }
            }
        });
    }

    drop(tx); // 关闭发送端

    // 收集并打印结果
    for result in rx {
        println!("{}", result);
    }
}