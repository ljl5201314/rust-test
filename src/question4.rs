// 从命令行参数接收一个文件路径，例如 input.txt。如果没有提供路径或文件无法打开，给出相应的错误提示并退出。
// 读取该文件的所有文本内容，统计文本中一共包含的字符数（不含换行符）与行数，并将结果写入 output.txt。
// 若 output.txt 文件已存在，可以选择直接覆盖或者追加，任选其一，但需要在程序里明确注释或说明处理方式。
use std::fs::{self, OpenOptions};
use std::io::Write;

pub fn process_file(file_path: &str) {
    //let args: Vec<String> = env::args().collect();
    // 读取文件内容    
    let content = match fs::read_to_string(file_path) {
        Ok(text) => text,
        Err(_) => {
            eprintln!("无法打开文件: {}", file_path);
            return;
        }
    };

    // 统计字符数（不包括换行符）
    let char_count = content.chars().filter(|&c| c != '\n').count();
    // 统计行数
    let line_count = content.lines().count();

    // 结果字符串
    let result = format!("字符数: {}\n行数: {}\n", char_count, line_count);

    // 写入 output.txt（覆盖方式）
    let mut output_file = match OpenOptions::new().write(true).create(true).truncate(true).open("output.txt") {
        Ok(file) => file,
        Err(_) => {
            eprintln!("无法创建或写入 output.txt");
            return;
        }
    };
    
    if let Err(_) = output_file.write_all(result.as_bytes()) {
        eprintln!("写入 output.txt 失败");
        return;
    }
}