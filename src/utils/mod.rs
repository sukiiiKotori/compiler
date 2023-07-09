pub mod float;
pub mod check;

#[macro_export]
macro_rules! log_println {
    () => {
        if get_settings().log {
            (println!());
        }
    };
    ($($arg:tt)*) => {
        use crate::get_settings;
        if get_settings().log {
            (println!($($arg)*));
        }
    };
}