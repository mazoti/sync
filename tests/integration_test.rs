mod processor;

#[test]
fn integration_test_check() {
    match processor::check::check("none", "nothing") {
        Err(err) => assert_eq!(err.code, processor::consts::ERROR_SOURCE_FOLDER),
        Ok(_) => panic!("ERROR => integration_test_check"),
    }
}

#[test]
fn integration_test_sync() {
    match processor::sync::sync("none", "nothing") {
        Err(err) => assert_eq!(err.code, processor::consts::ERROR_SOURCE_FOLDER),
        Ok(_) => panic!("ERROR => integration_test_sync"),
    }
}
