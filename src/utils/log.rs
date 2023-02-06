#[macro_export]
macro_rules! function_name {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);

        // Find and cut the rest of the path
        match &name[..name.len() - 3].rfind(':') {
            Some(pos) => &name[pos + 1..name.len() - 3],
            None => &name[..name.len() - 3],
        }
    }};
}

#[cfg(debug_assertions)]
#[macro_export]
macro_rules! log {
    ($prefix:literal, $msg:expr) => {
        // Because macro arguments don't have types, force type by assigning to variable which is typed
        let _p: &str = $prefix;
        println!("{:<70}\t{}", format!("{:<15} {:<30} {:<20}", format!("[{}]", $prefix), format!("({}:{})", file!(), line!()), format!("<{}>", crate::function_name!())), $msg);
    };
}

#[cfg(not(debug_assertions))]
#[macro_export]
macro_rules! log {
    ($prefix:literal, $msg:expr) => ();
}