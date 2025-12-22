fn main() {
    let cat = ("Furry McFurson", 3.5);
    // 解构元组，将第一个元素绑定到 name，第二个元素绑定到 age
    let (name, age) = cat;

    println!("{} is {} years old.", name, age);
}