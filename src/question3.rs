// 请从命令行读取一行字符串（例如 "apple banana pear banana apple banana"）。
// 使用空格进行拆分，统计每个单词出现的次数，并以从高到底的顺序输出。
// 如果出现次数相同，按单词本身的字典序从小到大排序输出。
use std::collections::HashMap;
//use std::io::{self, Write};

pub fn count_words(input: &str) -> Vec<(String, usize)> {
    //print!("请输入一行字符串信息: ");
    //io::stdout().flush().unwrap(); // 刷新输出缓冲区

    //let mut input = String::new();
    //io::stdin().read_line(&mut input).expect("读取输入失败");

    let mut word_count: HashMap<String, usize> = HashMap::new();
    
    // 统计单词出现次数
    for word in input.trim().split_whitespace() {
        *word_count.entry(word.to_string()).or_insert(0) += 1;
    }

    let mut word_list: Vec<(String, usize)> = word_count.into_iter().collect();
    
    // 按出现次数降序排列，次数相同，按字典序升序
    word_list.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    
    word_list
}