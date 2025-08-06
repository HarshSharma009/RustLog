/// Filters lines from a vector of strings based on a keyword.
/// Returns a new vector containing only the lines that contain the keyword.
use std::vec::Vec;
pub fn filter_lines(lines: Vec<String>, keyword: &str) -> Vec<String> {
    lines
        .into_iter()
        .filter(|line| line.contains(keyword))
        .collect()
}


/// Tests for the filter_lines function.
/// This module contains unit tests to ensure the filtering functionality works as expected.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_lines() {
        let lines = vec![
            "INFO: Start".to_string(),
            "ERROR: Fail".to_string(),
            "WARN: Warn".to_string(),
        ];
        let result = filter_lines(lines, "ERROR");
        assert_eq!(result, vec!["ERROR: Fail"]);
    }

    #[test]
    fn test_filter_no_match() {
        let lines = vec![
            "INFO: Start".to_string(),
            "WARN: Warn".to_string(),
        ];
        let result = filter_lines(lines, "FATAL");
        assert!(result.is_empty());
    }
}
