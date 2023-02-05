#[macro_export]
macro_rules! log {
    ($prefix:literal, $msg:expr) => {
        // Because macro arguments don't have types, force type by assigning to variable which is typed
        let _p: &str = $prefix;
        println!("{:<40}\t{}", format!("{:<15} {}", format!("[{}]", $prefix), format!("({}:{})", file!(), line!())), $msg);
    };
}