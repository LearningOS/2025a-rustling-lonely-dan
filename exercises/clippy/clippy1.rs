// 导入精确的 π 常量
use std::f32::consts::PI;

fn main() {
    let radius = 5.00f32;

    // 方案1：使用直接乘法（Clippy 推荐的平方写法）
    let area = PI * radius * radius;

    // 方案2：使用 powf (若坚持用幂函数，Clippy 也接受 powf(2.0))
    // let area = PI * radius.powf(2.0);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    );
}