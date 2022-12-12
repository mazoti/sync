mod processor;

#[test]
fn integration_test_check() {
    match processor::check("none", "nothing") {
        Err(err) => assert_eq!(err.code, 13), // pub const ERROR_SOURCE_FOLDER: i32 = 13;
        Ok(_) => panic!("ERROR => integration_test_check"),
    }
}

#[test]
fn integration_test_sync() {
    match processor::sync("none", "nothing") {
        Err(err) => assert_eq!(err.code, 13), // pub const ERROR_SOURCE_FOLDER: i32 = 13;
        Ok(_) => panic!("ERROR => integration_test_sync"),
    }
}
