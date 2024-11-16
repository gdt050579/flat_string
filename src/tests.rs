use crate::FlatString;

#[test]
fn check_create_empty() {
    let s = FlatString::<10>::new();
    assert_eq!(s.len(), 0);
    assert_eq!(s.chars_count(), 0);
    assert_eq!(s.as_str(), "");
}
#[test]
fn check_create_with_str() {
    let s = FlatString::<10>::from_str("Hello");
    assert_eq!(s.len(), 5);
    assert_eq!(s.chars_count(), 5);
    assert_eq!(s.as_str(), "Hello");
}

#[test]
fn check_create_with_large_str() {
    let s = FlatString::<10>::from_str("Hello world from Rust");
    assert_eq!(s.len(), 10);
    assert_eq!(s.chars_count(), 10);
    assert_eq!(s.as_str(), "Hello worl");
}

#[test]
fn check_create_with_unicode_string_that_fits() {
    let s = FlatString::<30>::from_str("こんにちは世界");
    assert_eq!(s.len(), 21);
    assert_eq!(s.chars_count(), 7);
    assert_eq!(s.as_str(), "こんにちは世界");
}

#[test]
fn check_create_with_unicode_string_that_does_not_fit() {
    let s = FlatString::<10>::from_str("こんにちは世界");
    assert_eq!(s.len(), 9);
    assert_eq!(s.chars_count(), 3);
    assert_eq!(s.as_str(), "こんに");
}

#[test]
fn check_with_default_size() {
    let s = FlatString::<14>::from_str("zăpadă la școală");
    assert_eq!(s.capacity(), 14);
    assert_eq!(s.len(), 14);
    assert_eq!(s.chars_count(), 11);
    assert_eq!(s.as_str(), "zăpadă la ș");
}