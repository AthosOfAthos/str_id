use crate::Name;
use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::Visitor;

impl Serialize for Name {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(self.as_str())
    }
}

struct NameVisitor {}

impl NameVisitor {
    fn deserialize<E: serde::de::Error>(self, name: &str) -> Result<Name, E> {
        if name.is_empty() { return Err(E::custom("Name cannot be empty")) }
        if name.len() > Name::LENGTH { return Err(E::custom("Name too long")) }
        if name.contains('\0') { return Err(E::custom("Name may not contain null")) }
        
        Ok(Name::new(name))
    }
}

impl<'de> Visitor<'de> for NameVisitor {
    type Value = Name;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "A str between 1 and {} chars long with no null chars", Name::LENGTH)
    }

    fn visit_str<E: serde::de::Error>(self, v: &str) -> Result<Self::Value, E> {
        self.deserialize(v)
    }

    fn visit_borrowed_str<E: serde::de::Error>(self, v: &'de str) -> Result<Self::Value, E>{
        self.deserialize(v)
    }

    fn visit_string<E: serde::de::Error>(self, v: String) -> Result<Self::Value, E>{
        self.deserialize(&v)
    }
}

impl<'de> Deserialize<'de> for Name {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        deserializer.deserialize_str(NameVisitor {})
    }
}
