use rustlog::filter::filter_lines;

#[test]
fn filters_lines_correctly() {
    let lines = vec![
        "INFO: Startup".to_string(),
        "ERROR: Out of memory".to_string(),
    ];

    let result = filter_lines(lines, "ERROR");
    assert_eq!(result, vec!["ERROR: Out of memory"]);
}
