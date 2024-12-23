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
    assert_eq!(s, s2);
}

#[test]
fn check_clone() {
    let s = FlatString::<10>::from_str("Hello");
    let s2 = s.clone();
    assert_eq!(s2.len(), 5);
    assert_eq!(s2.chars_count(), 5);
    assert_eq!(s2.as_str(), "Hello");
    assert_eq!(s, s2);
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

#[test]
fn check_display() {
    let s = FlatString::<10>::from_str("Hello");
    assert_eq!(format!("{}", s), "Hello");
}

#[test]
fn check_truncate() {
    let mut s = FlatString::<10>::from_str("Hello");
    s.truncate(3);
    assert_eq!(s.as_str(), "Hel");
    s.truncate(300);
    assert_eq!(s.as_str(), "Hel");
}

#[test]
#[should_panic]
fn check_truncate_panic() {
    let mut s = FlatString::<10>::from_str("こんにちは");
    s.truncate(1);
}

#[test]
fn check_pop() {
    let mut s = FlatString::<10>::from_str("Hi");
    assert_eq!(s.pop(), Some('i'));
    assert_eq!(s.chars, 1);
    assert_eq!(s.len, 1);
    assert_eq!(s.pop(), Some('H'));
    assert_eq!(s.chars, 0);
    assert_eq!(s.len, 0);
    assert_eq!(s.pop(), None);
    assert_eq!(s.chars, 0);
    assert_eq!(s.len, 0);

    s = FlatString::<10>::from_str("sこmんe");
    assert_eq!(s.pop(), Some('e'));
    assert_eq!(s.chars, 4);
    assert_eq!(s.len, 8);
    assert_eq!(s.pop(), Some('ん'));
    assert_eq!(s.chars, 3);
    assert_eq!(s.len, 5);
    assert_eq!(s.pop(), Some('m'));
    assert_eq!(s.chars, 2);
    assert_eq!(s.len, 4);
    assert_eq!(s.pop(), Some('こ'));
    assert_eq!(s.chars, 1);
    assert_eq!(s.len, 1);
    assert_eq!(s.pop(), Some('s'));
    assert_eq!(s.chars, 0);
    assert_eq!(s.len, 0);
    assert_eq!(s.pop(), None);
    assert_eq!(s.chars, 0);
    assert_eq!(s.len, 0);
}

#[test]
fn check_insert_middle_no_size_exceed() {
    let mut s = FlatString::<10>::from_str("abcde");
    s.insert(3, "012");
    assert_eq!(s.as_str(), "abc012de");
    assert_eq!(s.chars, 8);
    assert_eq!(s.len, 8);
}

#[test]
fn check_insert_start_no_size_exceed() {
    let mut s = FlatString::<10>::from_str("abcde");
    s.insert(0, "012");
    assert_eq!(s.as_str(), "012abcde");
    assert_eq!(s.chars, 8);
    assert_eq!(s.len, 8);
}

#[test]
fn check_insert_middle_exceeding_size() {
    let mut s = FlatString::<10>::from_str("abcde");
    s.insert(2, "0123456789");
    assert_eq!(s.as_str(), "ab01234567");
    assert_eq!(s.chars, 10);
    assert_eq!(s.len, 10);
}

#[test]
fn check_insert_middle_exact_size() {
    let mut s = FlatString::<10>::from_str("abcde");
    s.insert(5, "01234");
    assert_eq!(s.as_str(), "abcde01234");
    assert_eq!(s.chars, 10);
    assert_eq!(s.len, 10);
}

#[test]
fn check_insert_middle_unicode() {
    let mut s = FlatString::<10>::from_str("Hello");
    // insert unicode text in the middle
    s.insert(1, "んこ");
    assert_eq!(s.as_str(), "Hんこell");
    assert_eq!(s.chars, 6);
    assert_eq!(s.len, 10);
}

#[test]
fn check_insert_unicode_last() {
    // should not change the string
    // because it would exceeed SIZE
    let mut s = FlatString::<10>::from_str("Hんこell");
    s.insert(9, "こ");
    assert_eq!(s.as_str(), "Hんこell");
    assert_eq!(s.chars, 6);
    assert_eq!(s.len, 10);
}

#[test]
fn check_insert_unicode_start() {
    let mut s = FlatString::<10>::from_str("Hんこell");
    // insert in the start
    s.insert(0, "ABC");
    assert_eq!(s.as_str(), "ABCHんこ");
    assert_eq!(s.chars, 6);
    assert_eq!(s.len, 10);
}

#[test]
fn check_insert_unicode_last_not_fitting() {
    // simulate case when the last char is a unicode and
    // cannot be entirely fit in the data array and should be dropped
    let mut s = FlatString::<10>::from_str("ABCHんこ");
    s.insert(0, "01");
    assert_eq!(s.as_str(), "01ABCHん");
    assert_eq!(s.chars, 7);
    assert_eq!(s.len, 9);

    let mut s = FlatString::<10>::from_str("ABCHんこ");
    s.insert(0, "0");
    assert_eq!(s.as_str(), "0ABCHん");
    assert_eq!(s.chars, 6);
    assert_eq!(s.len, 8);
}

#[test]
fn check_insert_unicode_add_last_not_fitting() {
    // simulate case when the last char is a unicode and
    // cannot be entirely fit in the data array and should be dropped
    let mut s = FlatString::<5>::from_str("ABCDE");
    s.insert(3, "ん");
    assert_eq!(s.as_str(), "ABCDE");
    assert_eq!(s.chars, 5);
    assert_eq!(s.len, 5);
}

#[test]
fn check_insert_char() {
    let mut s = FlatString::<6>::from_str("Hello");

    s.insert_char(5, '!');
    assert_eq!(s.as_str(), "Hello!");
    s.insert_char(1, 'E');
    assert_eq!(s.as_str(), "HEello");
    s.insert_char(1, 'ん');
    assert_eq!(s.as_str(), "HんEe");
    s.insert_char(5, 'ん');
    assert_eq!(s.as_str(), "HんEe");
    s.insert_char(4, 'こ');
    assert_eq!(s.as_str(), "HんEe");
}

#[test]
#[should_panic]
fn check_insert_char_panic_idx() {
    let mut s = FlatString::<6>::from_str("Hello");
    s.insert_char(7, '!');
    assert_eq!(s.as_str(), "Hello!");
}

#[test]
#[should_panic]
fn check_insert_char_panic_middle_char() {
    let mut s = FlatString::<6>::from_str("HんEe");
    s.insert_char(2, '!');
    assert_eq!(s.as_str(), "Hello!");
}

#[test]
fn check_remove() {
    let mut s = FlatString::<7>::from_str("aんbcd");
    assert_eq!(s.remove(1), 'ん');
    assert_eq!(s.as_str(), "abcd");

    assert_eq!(s.remove(3), 'd');
    assert_eq!(s.as_str(), "abc");

    assert_eq!(s.remove(0), 'a');
    assert_eq!(s.as_str(), "bc");

    assert_eq!(s.remove(0), 'b');
    assert_eq!(s.as_str(), "c");

    assert_eq!(s.remove(0), 'c');
    assert_eq!(s.as_str(), "");
}

#[test]
#[should_panic]
fn check_remove_panic_idx() {
    let mut s = FlatString::<20>::from_str("aんbcd");
    s.remove(10);
}

#[test]
#[should_panic]
fn check_remove_panic_middle() {
    let mut s = FlatString::<20>::from_str("aんbcd");
    s.remove(2);
}
