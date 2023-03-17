
#[cfg(feature = "serde")]
mod serialize;

/// A small fixed length string 
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Name {
    array: [u8; Self::LENGTH],
}

const NULL: u8 = b'\0';

impl Name {
    pub const LENGTH: usize = 16;

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

    pub const fn len(&self) -> usize {
        let mut index = 0;
        while index <= Self::LENGTH {
            if self.array[index] == NULL {
                return index;
            }
            index += 1;
        }
        return Self::LENGTH;
    }

    pub const fn as_str(&self) -> &str {
        // This assumes Name::new was fed a valid UTF8 str
        unsafe {
            std::str::from_utf8_unchecked(&*std::ptr::slice_from_raw_parts(self.array.as_ptr(), self.len()))
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

impl std::fmt::Debug for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::fmt::Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl std::hash::Hash for Name {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write(&self.array);
    }
}
