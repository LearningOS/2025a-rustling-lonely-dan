#[derive(Debug)]
enum Message {
    // 定义各变体及其携带的数据
    Quit, // 单元变体，无数据
    Echo(String), // 元组变体，携带String类型
    Move { x: i32, y: i32 }, // 结构体变体，携带x和y坐标
    ChangeColor(u8, u8, u8), // 元组变体，携带三个u8类型的RGB值
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}