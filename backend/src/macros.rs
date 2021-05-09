#[macro_export]
macro_rules! code_loc {
    () => {
        format!("{}:{}", file!(), line!())
    };
}
