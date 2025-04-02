use std::env;
use crate::question1::fizzbuzz;
use crate::question2::Student;
use crate::question3::count_words;
use crate::question4::process_file;
use crate::question5::parallel_download;
use crate::question6::search_in_directory;

mod question1;
mod question2;
mod question3;
mod question4;
mod question5;
mod question6;
fn main() {
    //question1
    let args: Vec<String> = env::args().collect();
    let n: i32 = args.get(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(5);

    fizzbuzz(n);

    //question2
    let student = Student::new("Alice", 18, 95.5);
    student.show();
    println!("Passed? {}", student.is_passed());

    //question3
    let input = "apple banana pear banana apple banana";
    let word_count = count_words(input);
    for (word, count) in word_count {
        println!("{}: {}", word, count);
    }

    //question4
    let file_path = "example.txt";
    process_file(file_path);


    //question5
    parallel_download();

    //question6
    search_in_directory("example".to_string(), ".".to_string(), true);

}

#[cfg(test)]
mod tests {
    use std::fs;
    use crate::question1::fizzbuzz;
    use crate::question2::Student;
    use crate::question3::count_words;
    use crate::question4::process_file;
    use crate::question5::parallel_download;
    use crate::question6::search_in_directory;
    use tempfile::tempdir;


    //question1
    #[test]
    fn test_fizzbuzz() {
        // n=5
        let result = fizzbuzz(5);
        let expected = "1\n2\nFizz\n4\nBuzz\n";
        assert_eq!(result, expected);
    }
    #[test]
    fn test_fizzbuzz_with_n() {
        // n=10
        let result = fizzbuzz(10);
        let expected = "1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\nBuzz\n";
        assert_eq!(result, expected);
    }

    //question2
    #[test]
    fn test_student_creation() {
        let student = Student::new("Alice", 18, 95.5);
        assert_eq!(student.name, "Alice");
        assert_eq!(student.age, 18);
        assert_eq!(student.score, 95.5);
    }
    #[test]
    fn test_student_passed() {
        let student = Student::new("Bob", 20, 70.0);
        assert!(student.is_passed());

        let student2 =Student::new("Charlie", 22, 50.0);
        assert!(!student2.is_passed());
    }
    #[test]
    // fn test_student_show() {
    //     let student = Student::new("Alice", 18, 95.5);
    //     let mut output = Vec::new();
    //     student.show_with_write(&mut output);
    //     let expected_info = "Name: Alice, Age: 18, Score: 95.5";
    //     assert_eq!(String::from_utf8(output).unwrap(), expected_info);
    // }

    //question3
    #[test]
    fn test_word_count() {
        let input = "apple banana pear banana apple banana";
        let expected = vec![
            ("banana".to_string(), 3),
            ("apple".to_string(), 2),
            ("pear".to_string(), 1),
        ];
        
        let result = count_words(input);

        assert_eq!(result, expected);
    }

    //question4
    #[test]
    fn test_file_processing() {
        let content = "apple banana pear\nbanana apple\n";
        let file_path = "test.txt";

        // 写入测试文件
        fs::write(file_path, content).expect("Unable to write file");

        // 调用文件处理
        process_file(file_path);

        //读输入文件并验证
        let output_content = fs::read_to_string("output.txt").expect("Unable to read output.txt");
        let expected_output = "字符数: 24\n行数: 2\n";
        assert_eq!(output_content, expected_output);

        // 删除测试文件
        fs::remove_file(file_path).expect("Unable to remove file");
        fs::remove_file("output.txt").expect("Unable to remove output.txt");
    }

    //question5
    #[test]
    fn test_parallel_download() {
        parallel_download();
    }
    //question6
    #[test]
    fn test_search_in_directory() {
        let temp_dir = tempfile::tempdir().unwrap(); // 创建临时目录
        let dir_path = temp_dir.path().to_str().unwrap().to_string();

        // 创建测试文件
        let file1_path = temp_dir.path().join("file1.txt");
        fs::write(file1_path.clone(), "hello world\nhello rust").unwrap();

        let file2_path = temp_dir.path().join("file2.txt");
        fs::write(file2_path.clone(), "world rust\nhello").unwrap();

        // 执行搜索
        let keyword = "hello".to_string();
        let case_sensitive = false;
        search_in_directory(keyword.clone(), dir_path.clone(), case_sensitive);

        // 验证结果（可以通过捕获标准输出来验证）
        // 这里假设搜索结果正确输出到控制台
    }
}
