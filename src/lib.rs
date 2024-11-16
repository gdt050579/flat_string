#[cfg(test)]
mod tests;

use std::ops::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct FlatString<const SIZE: usize = 14> {
    data: [u8; SIZE],
    len: u8,
    chars: u8,
}
impl<const SIZE: usize> FlatString<SIZE> {
    pub fn new() -> Self {
        assert!(SIZE > 0, "SIZE must be greater than 0");
        assert!(SIZE < 256, "SIZE must be less than 256");
        Self {
            data: [0; SIZE],
            len: 0,
            chars: 0,
        }
    }
    pub fn from_str(text: &str) -> Self {
        let mut this = Self::new();
        this.push_str(text);
        this
    }
    #[inline(always)]
    pub fn clear(&mut self) {
        self.len = 0;
        self.chars = 0;
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len as usize
    }
    #[inline(always)]
    pub fn chars_count(&self) -> usize {
        self.chars as usize
    }
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        SIZE
    }
    // copy the string only if it fits the available space or return false otherwise
    fn add_entire_string(&mut self, text: &str) -> bool {
        let len = text.len();
        if len + self.len as usize <= SIZE {
            self.data[self.len as usize..self.len as usize + len].copy_from_slice(text.as_bytes());
            self.len += len as u8;
            self.chars += text.chars().count() as u8;
            true
        } else {
            false
        }
    }
    // fills the buffer with as much characters as possible
    fn fill_with_str(&mut self, text: &str) {
        let mut poz = 0;
        let mut count_chars = 0;
        let len = self.len as usize;
        for (i, _) in text.char_indices() {
            if i + len > SIZE {
                break;
            }
            poz = i;
            count_chars += 1;
        }
        // we count the number of characters that fit in the buffer
        // the first character is already counted (as its pozition will be 0)
        if count_chars > 1 {
            let bytes = text[..poz].as_bytes();
            self.data[len..len + poz].copy_from_slice(bytes);
            self.len += poz as u8;
            self.chars += (count_chars - 1) as u8;
        }
    }
    #[inline(always)]
    pub fn push_str(&mut self, text: &str) {
        // try the fast method first
        if !self.add_entire_string(text) {
            // if it fails, copy as much characters as possible
            self.fill_with_str(text);
        }
    }
    #[inline(always)]
    pub fn push(&mut self, c: char) {
        let mut bytes = [0; 8];
        self.push_str(c.encode_utf8(&mut bytes));
    }

    #[inline(always)]
    pub fn try_push_str(&mut self, text: &str) -> Option<&str> {
        if self.add_entire_string(text) {
            Some(self.as_str())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn try_push(&mut self, c: char) -> Option<&str> {
        let mut bytes = [0; 8];
        if self.add_entire_string(c.encode_utf8(&mut bytes)) {
            Some(self.as_str())
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn set(&mut self, text: &str) {
        self.clear();
        self.push_str(text);
    }
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        if self.len == 0 {
            ""
        } else {
            unsafe { std::str::from_utf8_unchecked(&self.data[..self.len as usize]) }
        }
    }
}

impl Deref for FlatString {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}
