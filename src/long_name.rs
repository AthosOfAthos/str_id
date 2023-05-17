
/// For when size matters
#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct LongName([u8; Self::LENGTH]);

const NULL: u8 = b'\0';

impl LongName {
	pub const LENGTH: usize = 128;

	pub const fn new(name: &str) -> Self {
		assert!(name.len() > 0, "Failed to create LongName: Cannot be empty");
		assert!(name.len() <= Self::LENGTH, "Failed to create LongName: Length may not exceed 128");

		let name = name.as_bytes();
		let mut array = [0; Self::LENGTH];
		
		let mut index = 0;
		while index < name.len() {
			assert!(name[index] != NULL, "Failed to create LongName: Cannot contain NULL chars");
			array[index] = name[index];
			index += 1;
		}
		
		LongName(array)
	}

	pub const fn len(&self) -> usize {
		let mut index = 0;
		while index < Self::LENGTH {
			if self.0[index] == NULL {
				return index;
			}

			index += 1;
		}

		return Self::LENGTH;
	}

	pub const fn as_str(&self) -> &str {
		unsafe {
			core::str::from_utf8_unchecked(&*core::ptr::slice_from_raw_parts(self.0.as_ptr(), self.len()))
		}
	}
}

impl PartialEq for LongName {
    fn eq(&self, other: &Self) -> bool {
		for index in 0..Self::LENGTH {
			if (self.0[index] == NULL) && (other.0[index] == NULL) {
				return true;
			}

			if self.0[index] != other.0[index] {
				return false;
			}
		}
		true
    }
}
impl Eq for LongName {}

impl core::fmt::Debug for LongName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", self.as_str())
    }
}

impl core::fmt::Display for LongName {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "{}", self.as_str())
    }
}

impl core::hash::Hash for LongName {
	fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
		state.write(&self.0);
	}
}

#[cfg(test)]
mod tests {
	use crate::LongName;

	#[test]
	fn from_str() {
		const STR: &str = "~/Projects/Test/example_path.psd";
		const PATH: LongName = LongName::new(STR);

		assert_eq!(STR.len(), PATH.len());
		assert_eq!(STR, PATH.as_str());
	}

	#[test]
	fn eq() {
		const IMAGE_PATH: LongName = LongName::new("Textures/image.png");
		const MESH_PATH: LongName = LongName::new("Models/TestCube.obj");

		assert_ne!(IMAGE_PATH, MESH_PATH);
		assert_eq!(IMAGE_PATH, IMAGE_PATH);
		let mesh_path = MESH_PATH;
		assert_eq!(MESH_PATH, mesh_path);
	}

	#[test]
	fn max_len() {
		LongName::new("12345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678");
	}
	
	#[test]
	#[should_panic]
	fn too_long() {
		LongName::new("123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781234567812345678123456781");
	}

	#[test]
	#[should_panic]
	fn empty() {
		LongName::new("");
	}
}
