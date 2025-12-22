mod sausage_factory {
    // 保持私有，仅模块内可访问
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    // 声明为公共函数，允许模块外访问
    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}