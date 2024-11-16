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

#[test]
fn check_clear() {
    let mut s = FlatString::<10>::from_str("Hello");
    s.clear();
    assert_eq!(s.len(), 0);
    assert_eq!(s.chars_count(), 0);
    assert_eq!(s.as_str(), "");
}

#[test]
fn check_push_str() {
    let mut s = FlatString::<10>::new();
    s.push_str("Hello");
    assert_eq!(s.len(), 5);
    assert_eq!(s.chars_count(), 5);
    assert_eq!(s.as_str(), "Hello");
    s.push_str(" World !");
    assert_eq!(s.len(), 10);
    assert_eq!(s.chars_count(), 10);
    assert_eq!(s.as_str(), "Hello Worl");
}

#[test]
fn check_try_push_str() {
    let mut s = FlatString::<10>::new();
    assert_eq!(s.try_push_str("Hello"), Some("Hello"));
    assert_eq!(s.len(), 5);
    assert_eq!(s.chars_count(), 5);
    assert_eq!(s.as_str(), "Hello");
    assert_eq!(s.try_push_str("World !"), None);
    assert_eq!(s.len(), 5);
    assert_eq!(s.chars_count(), 5);
    assert_eq!(s.as_str(), "Hello");
}

#[test]
fn check_push_char() {
    let mut s = FlatString::<10>::new();
    s.push('H');
    assert_eq!(s.len(), 1);
    assert_eq!(s.chars_count(), 1);
    assert_eq!(s.as_str(), "H");
    s.push('e');
    assert_eq!(s.len(), 2);
    assert_eq!(s.chars_count(), 2);
    assert_eq!(s.as_str(), "He");
    s.push('l');
    assert_eq!(s.len(), 3);
    assert_eq!(s.chars_count(), 3);
    assert_eq!(s.as_str(), "Hel");
    s.push('l');
    assert_eq!(s.len(), 4);
    assert_eq!(s.chars_count(), 4);
    assert_eq!(s.as_str(), "Hell");
    s.push('o');
    assert_eq!(s.len(), 5);
    assert_eq!(s.chars_count(), 5);
    assert_eq!(s.as_str(), "Hello");
}

#[test]
fn check_try_push_char() {
    let mut s = FlatString::<5>::new();
    assert_eq!(s.try_push('H'), Some("H"));
    assert_eq!(s.len(), 1);
    assert_eq!(s.chars_count(), 1);
    assert_eq!(s.as_str(), "H");
    assert_eq!(s.try_push('e'), Some("He"));
    assert_eq!(s.len(), 2);
    assert_eq!(s.chars_count(), 2);
    assert_eq!(s.as_str(), "He");
    assert_eq!(s.try_push('l'), Some("Hel"));
    assert_eq!(s.len(), 3);
    assert_eq!(s.chars_count(), 3);
    assert_eq!(s.as_str(), "Hel");
    assert_eq!(s.try_push('l'), Some("Hell"));
    assert_eq!(s.len(), 4);
    assert_eq!(s.chars_count(), 4);
    assert_eq!(s.as_str(), "Hell");
    assert_eq!(s.try_push('o'), Some("Hello"));
    assert_eq!(s.len(), 5);
    assert_eq!(s.chars_count(), 5);
    assert_eq!(s.as_str(), "Hello");
    assert_eq!(s.try_push('!'), None);
    assert_eq!(s.len(), 5);
    assert_eq!(s.chars_count(), 5);
    assert_eq!(s.as_str(), "Hello");
}

#[test]
fn check_set() {
    let mut s = FlatString::<10>::from_str("Hello");
    s.set("World !");
    assert_eq!(s.len(), 7);
    assert_eq!(s.chars_count(), 7);
    assert_eq!(s.as_str(), "World !");
}

#[test]
fn check_set_partial() {
    let mut s = FlatString::<5>::from_str("Hello");
    s.set("World !");
    assert_eq!(s.len(), 5);
    assert_eq!(s.chars_count(), 5);
    assert_eq!(s.as_str(), "World");
}

#[test]
fn check_copy() {
    let s = FlatString::<10>::from_str("Hello");
    let s2 = s;
    assert_eq!(s2.len(), 5);
    assert_eq!(s2.chars_count(), 5);
    assert_eq!(s2.as_str(), "Hello");
    assert_eq!(s,s2);
}

#[test]
fn check_clone() {
    let s = FlatString::<10>::from_str("Hello");
    let s2 = s.clone();
    assert_eq!(s2.len(), 5);
    assert_eq!(s2.chars_count(), 5);
    assert_eq!(s2.as_str(), "Hello");
    assert_eq!(s,s2);
}

#[test]
fn check_memory_size() {
    let s = FlatString::<14>::from_str("Hello");
    assert_eq!(std::mem::size_of_val(&s), 16);
    let s = FlatString::<2>::from_str("Hello");
    assert_eq!(std::mem::size_of_val(&s), 4);
    assert_eq!(s.as_str(), "He");
}

#[test]
fn check_deref_from_str() {
    let s = FlatString::<10>::from_str("Hello");
    assert_eq!(s.len(), 5);
    assert_eq!(s.chars_count(), 5);
    assert_eq!(s.as_str(), "Hello");
    assert_eq!(s.starts_with("He"), true);
    assert_eq!(s.ends_with("lo"), true);
    assert_eq!(s.contains("ell"), true);
}