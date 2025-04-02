// 从命令行读取一个整数 n（若读取失败或没有输入则默认 n = 5）。
// 打印从 1 到 n 的所有整数，每行一个。
// 若该整数可以被 3 整除，则在数字后面附加输出 "Fizz"；若可以被 5 整除，则附加输出 "Buzz"；若同时满足可以被 3 和 5 整除的情况，则输出 "FizzBuzz"。
pub fn fizzbuzz(n: i32) -> String {
    let mut result = String::new();
    for i in 1..=n {
        match (i % 3, i % 5) {
            (0, 0) => result.push_str("FizzBuzz\n"),
            (0, _) => result.push_str("Fizz\n"),
            (_, 0) => result.push_str("Buzz\n"),
            _ => result.push_str(&format!("{}\n", i)),
        }
    }

    result
}