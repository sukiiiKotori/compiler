mod untils;
mod structures;
mod ast;
mod llvm_gen;
mod riscv_gen;
/*
编译器设置选项
*/
pub struct Settings {
    pub use_phi: bool,              // 使用phi指令
    pub optimise: bool,             // 开启优化
    pub debug: bool,                // 调试模式
    pub log: bool,                  // 打印日志
    pub all_allocs_in_entry: bool,  // 在入口处全部分配
}
static SETTINGS: Settings = Settings {
    use_phi: false,
    optimise: true,
    debug: false,
    log: false,
    all_allocs_in_entry: true,
};
pub fn get_settings() -> &'static Settings {
    &SETTINGS
}

#[macro_export]
macro_rules! log_println {
    () => {
        if get_settings().log {
            (println!());
        }
    };
    ($($arg:tt)*) => {
        if get_settings().log {
            (println!($($arg)*));
        }
    };
}


fn main() {
    println!("Hello, world!");
}