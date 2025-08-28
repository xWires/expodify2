use std::fs;

const DESTINATION: &str = "/tmp/expodify2-test";

#[test]
fn extract_media() {
    let _ = env_logger::builder().is_test(true).try_init();

    if !fs::exists(DESTINATION).unwrap() {
        fs::create_dir(DESTINATION).unwrap();
    }

    expodify2::extract("tests/resources/normal", DESTINATION).unwrap();

    assert_eq!(fs::read_dir(DESTINATION).unwrap().count(), 2);

    fs::remove_dir_all(DESTINATION).unwrap();
}

#[test]
fn duplicates() {
    let _ = env_logger::builder().is_test(true).try_init();

    if !fs::exists(DESTINATION).unwrap() {
        fs::create_dir(DESTINATION).unwrap();
    }

    expodify2::extract("tests/resources/duplicates", DESTINATION).unwrap();

    assert_eq!(fs::read_dir(DESTINATION).unwrap().count(), 4);

    fs::remove_dir_all(DESTINATION).unwrap();
}
