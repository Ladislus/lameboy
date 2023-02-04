#[macro_export]
macro_rules! log {
    ($prefix:literal, $msg:expr) => {
        // Because macro arguments don't have types, force type by assigning to variable which is typed
        let _p: &str = $prefix;

        let formatted_prefix = format!("[{}]", $prefix);
        let formatted_location = format!("({}:{})", file!(), line!());
        let informations = format!("{:<15} {}", formatted_prefix, formatted_location);
        println!("{:<40}\t{}", informations, $msg);
    };
}