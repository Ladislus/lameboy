#[macro_export]
macro_rules! log {
    ($prefix:literal, $msg:expr) => {
        let _p: &str = $prefix;
        println!("[{}] ({}/{}:{}) \t\t\t {}", $prefix, file!(), line!(), column!(), $msg);
    };
}