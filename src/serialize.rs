use crate::{Name, LongName};
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::Visitor;

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.as_str())
    }
}

struct NameVisitor;

impl<'de> Visitor<'de> for NameVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "A str between 1 and {} chars long with no NULL chars", Name::LENGTH)
    }

    fn visit_str<E: serde::de::Error>(self, name: &str) -> Result<Self::Value, E> {
        if name.is_empty() { return Err(E::custom("Name cannot be empty")) }
        if name.len() > Name::LENGTH { return Err(E::custom("Name too long")) }
        if name.contains('\0') { return Err(E::custom("Name may not contain null")) }
        
        Ok(Name::new(name))
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(NameVisitor {})
    }
}


impl Serialize for LongName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.as_str())
    }
}

struct LongNameVisitor;

impl<'de> Visitor<'de> for LongNameVisitor {
    type Value = LongName;

    fn expecting(&self, formatter: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(formatter, "A str between 1 and {} chars long with no NULL chars", LongName::LENGTH)
    }

    fn visit_str<E: serde::de::Error>(self, name: &str) -> Result<Self::Value, E> {
        if name.is_empty() { return Err(E::custom("LongName cannot be empty")) }
        if name.len() > LongName::LENGTH { return Err(E::custom("LongName too long")) }
        if name.contains('\0') { return Err(E::custom("LongName may not contain null")) }
        
        Ok(LongName::new(name))
    }
}

impl<'de> Deserialize<'de> for LongName {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(LongNameVisitor {})
    }
}
