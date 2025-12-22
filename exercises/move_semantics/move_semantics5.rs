fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x; // 此时 y 的作用域已结束（后续不再使用 y），可以创建 z
    *z += 1000;
    assert_eq!(x, 1200);
}