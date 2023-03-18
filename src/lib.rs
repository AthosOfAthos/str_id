#![no_std]

#[cfg(feature = "serde")]
mod serialize;

/// A small fixed length string that's cheap to create and compare.
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Name {
    array: [u8; Self::LENGTH],
}

const NULL: u8 = b'\0';

impl Name {
    pub const LENGTH: usize = 16;

    /// Creates a new Name from a string slice.
    /// Name must be valid UTF8 and be between 1..16 bytes long.
    /// # Example
    /// ```
    /// use str_id::Name;
    /// const AIR_ID: Name = Name::new("air");
    /// const STONE_ID: Name = Name::new("stone");
    /// ```
    pub const fn new(name: &str) -> Self {
        assert!(name.len() > 0, "Failed to create Name: Cannot be empty");
        assert!(name.len() <= Self::LENGTH, "Failed to create Name: Length may not exceed 16");

        let name = name.as_bytes();
        let mut array = [0; Self::LENGTH];
        let mut index = 0;
        while index < name.len() {
            assert!(name[index] != NULL, "Failed to create Name: May not contain NULL chars");
            array[index] = name[index];
            index += 1;
        }
        Name { array }
    }

    /// Returns the length of internal string **in bytes**.
    ///
    /// # Example
    ///
    /// ```
    /// use str_id::Name;
    ///
    /// let fancy_foo = Name::new("ƒoo");
    /// assert_eq!(4, fancy_foo.len());
    ///
    /// let bar = Name::new("bar");
    /// assert_eq!(3, bar.len());
    /// ```
    pub const fn len(&self) -> usize {
        let mut index = 0;
        while index < Self::LENGTH {
            if self.array[index] == NULL {
                return index;
            }
            index += 1;
        }
        return Self::LENGTH;
    }

    /// Returns a string slice representation of self.
    ///
    /// # Example
    ///
    /// ```
    /// let name = str_id::Name::new("foo");
    /// assert_eq!("foo", name.as_str());
    /// ```
    pub const fn as_str(&self) -> &str {
        // This assumes Name::new was fed a valid UTF8 str
        unsafe {
            core::str::from_utf8_unchecked(&*core::ptr::slice_from_raw_parts(self.array.as_ptr(), self.len()))
        }
    }
}

impl PartialEq for Name {
    fn eq(&self, other: &Name) -> bool {
        for index in 0..Self::LENGTH {
            if self.array[index] == NULL && other.array[index] == NULL {
                return true;
            }

            if self.array[index] != other.array[index] {
                return false;
            }
        }
        return true;
    }
}

impl Eq for Name {}

impl core::fmt::Debug for Name {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::fmt::Display for Name {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl core::hash::Hash for Name {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.array);
    }
}

#[cfg(test)]
mod tests {
    use crate::Name;
    
    #[test]
    fn odd_names() {
        Name::new("absentmindedness");
        Name::new("Old Hag Syndrome");
        Name::new("ありがとう");
        Name::new("WorkingAsIntendd");
    }

    #[test]
    fn eq() {
        const AIR: Name = Name::new("air");
        const STONE: Name = Name::new("stone");

        assert_eq!(AIR, Name::new("air"));
        assert_eq!(STONE, STONE);
        assert_ne!(AIR, STONE);
    }

    #[test]
    fn str() {
        const EXAMPLE_STR: &str = "example";
        const EXAMPLE: Name = Name::new(EXAMPLE_STR);
        
        assert_eq!(EXAMPLE.as_str(), EXAMPLE_STR);
        assert_eq!(EXAMPLE.len(), EXAMPLE_STR.len());
    }

    #[test]
    fn max_len() {
        assert_eq!(Name::new("biocompatibility").len(), 16);
    }
}
