pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

mod my_module {
    use super::Command;

    // 输入：Vec<(String, Command)>，输出：Vec<String>
    pub fn transformer(input: &Vec<(String, Command)>) -> Vec<String> {
        // 初始化输出向量，存储转换后的字符串
        let mut output: Vec<String> = vec![];
        for (string, command) in input.iter() {
            // 根据命令类型处理字符串
            let result = match command {
                Command::Uppercase => string.to_uppercase(),
                Command::Trim => string.trim().to_string(),
                Command::Append(n) => format!("{}{}", string, "bar".repeat(*n)),
            };
            output.push(result);
        }
        output
    }
}

#[cfg(test)]
mod tests {
    // 导入my_module中的transformer函数
    use super::my_module::transformer;
    use super::Command;

    #[test]
    fn it_works() {
        let output = transformer(vec![
            ("hello".into(), Command::Uppercase),
            (" all roads lead to rome! ".into(), Command::Trim),
            ("foo".into(), Command::Append(1)),
            ("bar".into(), Command::Append(5)),
        ]);
        assert_eq!(output[0], "HELLO");
        assert_eq!(output[1], "all roads lead to rome!");
        assert_eq!(output[2], "foobar");
        assert_eq!(output[3], "barbarbarbarbarbar");
    }
}