pub fn assert_error<T>(
    result: anyhow::Result<T>,
    expected_msg: &str,
) {
    if let Err(e) = result {
        assert_eq!(e.to_string(), expected_msg);
    } else {
        panic!("Expected an error, but got a success result");
    }
}