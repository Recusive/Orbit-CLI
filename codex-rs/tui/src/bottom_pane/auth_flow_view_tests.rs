use super::*;
use pretty_assertions::assert_eq;

#[test]
fn mask_for_display_short_input() {
    assert_eq!(mask_for_display("abc"), "***");
    assert_eq!(mask_for_display("abcd"), "****");
}

#[test]
fn mask_for_display_long_input() {
    let result = mask_for_display("sk-ant-api03-xxxxxxxxxxxxx");
    assert!(result.starts_with("sk-a"));
    assert!(result.contains('*'));
    // First 4 chars visible, rest masked.
    assert_eq!(&result[..4], "sk-a");
}

#[test]
fn mask_for_display_empty() {
    assert_eq!(mask_for_display(""), "");
}
