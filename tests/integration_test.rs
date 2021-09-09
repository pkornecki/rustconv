use std::path::PathBuf;

#[tokio::test]
async fn conversion_works() {
    let input = PathBuf::from("testdata/test-input.csv");
    let output = PathBuf::from("testdata/test-output.csv");
    let hotels = PathBuf::from("testdata/test-hotels.json");

    rustconv::run(input, output.clone(), hotels).await.unwrap();

    let actual = std::fs::read_to_string(output).unwrap();
    let expected = std::fs::read_to_string(PathBuf::from("testdata/expected-output.csv")).unwrap();

    assert_eq!(actual, expected);
}
