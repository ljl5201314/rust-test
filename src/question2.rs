// 定义一个 Student 结构体，包含以下字段：name、age、score
// 实现以下功能：
// - new(name: &str, age: u8, score: f32) -> Student：返回一个新的学生实例。
// - show(&self)：打印 Student 的信息，格式如 Name: Alice, Age: 18, Score: 95.5。
// - is_passed(&self) -> bool：如果 score >= 60.0 则返回 true，否则返回 false。
pub struct Student {
    pub name: String,
    pub age: u8,
    pub score: f32,
}

impl Student {
    // 创建新的 Student 实例
    pub fn new(name: &str, age: u8, score: f32) -> Self {
        Self {
            name: name.to_string(),
            age,
            score,
        }
    }

    // 显示学生信息
    pub fn show(&self) {
        println!("Name: {}, Age: {}, Score: {:.1}", self.name, self.age, self.score);
    }

    pub fn show_with_write(&self, writer: &mut impl std::io::Write) {
        writeln!(writer, "Name: {}, Age: {}, Score: {:.1}", self.name, self.age, self.score).unwrap();
    }
    // 判断是否及格
    pub fn is_passed(&self) -> bool {
        self.score >= 60.0
    }
}