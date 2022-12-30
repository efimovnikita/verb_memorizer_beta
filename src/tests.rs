use crate::library::*;

#[test]
fn test_is_two_forms_correct() {
    // Test that the function returns Ok for a valid input
    assert_eq!(is_two_forms_correct("abc def"), Ok("abc def".to_string()));

    // Test that the function returns Err for an input with no whitespace
    assert!(is_two_forms_correct("abcdef").is_err());

    // Test that the function returns Err for an input with only one character
    assert!(is_two_forms_correct("a").is_err());

    // Test that the function returns Ok for valid input
    assert_eq!(is_two_forms_correct("Abc, def"), Ok("Abc def".to_string()));

    assert_eq!(is_two_forms_correct("Abc,def"), Ok("Abc def".to_string()));

    // Test that the function returns Ok for valid input
    assert_eq!(
        is_two_forms_correct("Abc     def"),
        Ok("Abc def".to_string())
    );

    // Test that the function returns Err for an empty input
    assert!(is_two_forms_correct("").is_err());
}

#[test]
fn test_validate_correct() {
    let verb = IrregularVerb::new("see".to_string(), "saw".to_string(), "seen".to_string());
    let result = validate("saw".to_string(), &&verb, "seen".to_string());
    assert!(result.0);
    assert_eq!(result.1, "".to_string());
}

#[test]
fn test_validate_incorrect_past() {
    let verb = IrregularVerb::new("see".to_string(), "saw".to_string(), "seen".to_string());
    let result = validate("see".to_string(), &&verb, "seen".to_string());
    assert!(!result.0);
    assert_eq!(result.1, "see - saw - seen".to_string());
}

#[test]
fn test_validate_incorrect_past_participle() {
    let verb = IrregularVerb::new("see".to_string(), "saw".to_string(), "seen".to_string());
    let result = validate("saw".to_string(), &&verb, "see".to_string());
    assert!(!result.0);
    assert_eq!(result.1, "see - saw - seen".to_string());
}
