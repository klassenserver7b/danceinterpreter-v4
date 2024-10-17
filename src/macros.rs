#[macro_export]
macro_rules! test_file {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/resources/test/", $fname)
    };
}
#[macro_export]
macro_rules! res_file {
    ($fname:expr) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/resources/main/", $fname)
    };
}
