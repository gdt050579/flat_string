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
    /// Create a new FlatString with a fixed size
    /// 
    /// # Panics
    /// - If SIZE is 0 or greater than 255
    /// 
    /// # Example
    /// ```rust
    /// use flat_string::FlatString;
    /// let s = FlatString::<10>::new();
    /// ```
    pub fn new() -> Self {
        assert!(SIZE > 0, "SIZE must be greater than 0");
        assert!(SIZE < 256, "SIZE must be less than 256");
        Self {
            data: [0; SIZE],
            len: 0,
            chars: 0,
        }
    }

    /// Create a new FlatString from a string slice
    /// If the string slice is larger than the available space, only the first characters that fit will be copied
    /// 
    /// # Panics
    /// - If SIZE is 0 or greater than 255
    /// 
    /// # Example
    /// ```rust
    /// use flat_string::FlatString;
    /// let s = FlatString::<10>::from_str("Hello");
    /// ```
    pub fn from_str(text: &str) -> Self {
        let mut this = Self::new();
        this.push_str(text);
        this
    }

    /// Clears the content of the FlatString. This operation does not deallocate the memory, not it does not clear the content o the string. It only resets the length and characters count to 0.
    #[inline(always)]
    pub fn clear(&mut self) {
        self.len = 0;
        self.chars = 0;
    }

    /// Returns the length of the string in bytes. This operation is performed in O(1) time.
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len as usize
    }

    /// Returns true if the string is empty, false otherwise. This operation is performed in O(1) time.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Returns the number of characters in the string. This operation is performed in O(1) time.
    #[inline(always)]
    pub fn chars_count(&self) -> usize {
        self.chars as usize
    }

    /// Returns the capacity of the FlatString. This operation is performed in O(1) time.
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

    /// Appends a string slice to the FlatString. If the string slice is larger than the available space, only the first characters that fit will be copied.
    /// 
    /// # Example
    /// ```rust
    /// use flat_string::FlatString;
    /// let mut s = FlatString::<10>::new();
    /// s.push_str("Hello");
    /// ```
    #[inline(always)]
    pub fn push_str(&mut self, text: &str) {
        // try the fast method first
        if !self.add_entire_string(text) {
            // if it fails, copy as much characters as possible
            self.fill_with_str(text);
        }
    }

    /// Appends a character to the FlatString. If the character does not fit in the available space, it will not be copied. 
    /// 
    /// # Example
    /// ```rust
    /// use flat_string::FlatString;
    /// let mut s = FlatString::<10>::new();
    /// s.push('H');
    /// ```
    #[inline(always)]
    pub fn push(&mut self, c: char) {
        let mut bytes = [0; 8];
        self.push_str(c.encode_utf8(&mut bytes));
    }

    /// Tries to append a string slice to the FlatString. If the string slice can fit in the available space, it will be copied and Some(&str) will be returned. Otherwise, None will be returned and ths string will remain unchanged.    
    /// 
    /// # Example
    /// ```rust
    /// use flat_string::FlatString;
    /// let mut s = FlatString::<10>::new();
    /// assert_eq!(s.try_push_str("Hello"), Some("Hello"));
    /// assert_eq!(s.try_push_str(" Wor"), Some("Hello Wor"));
    /// assert_eq!(s.try_push_str("ld !"), None); // the string does not fit
    /// ```
    #[inline(always)]
    pub fn try_push_str(&mut self, text: &str) -> Option<&str> {
        if self.add_entire_string(text) {
            Some(self.as_str())
        } else {
            None
        }
    }

    /// Tries to append a character to the FlatString. If the character can fit in the available space, it will be copied and Some(&str) will be returned. Otherwise, None will be returned and ths string will remain unchanged.
    /// 
    /// # Example
    /// ```rust
    /// use flat_string::FlatString;
    /// let mut s = FlatString::<5>::new();
    /// assert_eq!(s.try_push('H'), Some("H"));
    /// assert_eq!(s.try_push('e'), Some("He"));
    /// assert_eq!(s.try_push('l'), Some("Hel"));
    /// assert_eq!(s.try_push('l'), Some("Hell"));
    /// assert_eq!(s.try_push('o'), Some("Hello"));
    /// assert_eq!(s.try_push('!'), None); // the character does not fit
    /// ```
    #[inline(always)]
    pub fn try_push(&mut self, c: char) -> Option<&str> {
        let mut bytes = [0; 8];
        if self.add_entire_string(c.encode_utf8(&mut bytes)) {
            Some(self.as_str())
        } else {
            None
        }
    }

    /// Sets the content of the FlatString to a string slice. If the string slice is larger than the available space, only the first characters that fit will be copied.
    /// 
    /// # Example
    /// ```rust
    /// use flat_string::FlatString;
    /// let mut s = FlatString::<10>::new();
    /// s.set("Hello");
    /// ```
    #[inline(always)]
    pub fn set(&mut self, text: &str) {
        self.clear();
        self.push_str(text);
    }

    /// Returns the content of the FlatString as a string slice. This operation is performed in O(1) time.
    /// 
    /// # Example
    /// ```rust
    /// use flat_string::FlatString;
    /// let s = FlatString::<10>::from_str("Hello");
    /// assert_eq!(s.as_str(), "Hello");
    /// ```
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        if self.len == 0 {
            ""
        } else {
            unsafe { std::str::from_utf8_unchecked(&self.data[..self.len as usize]) }
        }
    }
}

impl<const SIZE: usize> Deref for FlatString<SIZE> {
    type Target = str;
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl<const SIZE: usize> Default for FlatString<SIZE> {
    fn default() -> Self {
        Self::new()
    }
}

impl<const SIZE: usize> std::fmt::Display for FlatString<SIZE> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}
