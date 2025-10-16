use std::fs;
use expodify2::Extractor;

const DESTINATION: &str = "/tmp/expodify2-test";

#[test]
fn extract_media() {
    let _ = env_logger::builder().is_test(true).try_init();

    if !fs::exists(DESTINATION).unwrap() {
        fs::create_dir(DESTINATION).unwrap();
    }

    Extractor::builder()
        .source("tests/resources/normal")
        .destination(DESTINATION)
        .build()
        .unwrap()
        .extract()
        .unwrap();

    assert_eq!(fs::read_dir(DESTINATION).unwrap().count(), 2);

    fs::remove_dir_all(DESTINATION).unwrap();
}

#[test]
fn duplicates() {
    let _ = env_logger::builder().is_test(true).try_init();

    if !fs::exists(DESTINATION).unwrap() {
        fs::create_dir(DESTINATION).unwrap();
    }

    Extractor::builder()
        .source("tests/resources/duplicates")
        .destination(DESTINATION)
        .build()
        .unwrap()
        .extract()
        .unwrap();

    assert_eq!(fs::read_dir(DESTINATION).unwrap().count(), 4);

    fs::remove_dir_all(DESTINATION).unwrap();
}
