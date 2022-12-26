use super::*;

#[test]
fn test_is_two_forms_correct() {
    // Test that the function returns Ok for a valid input
    assert_eq!(is_two_forms_correct("abc def"), Ok("abc def".to_string()));

    // Test that the function returns Err for an input with no whitespace
    assert!(is_two_forms_correct("abcdef").is_err());

    // Test that the function returns Err for an input with only one character
    assert!(is_two_forms_correct("a").is_err());

    // Test that the function returns Err for an input with multiple consecutive whitespaces
    assert!(is_two_forms_correct("Abc, def").is_err());

    // Test that the function returns Err for an empty input
    assert!(is_two_forms_correct("").is_err());
}
