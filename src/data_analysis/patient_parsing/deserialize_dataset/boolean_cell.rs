use std::fmt::{self, Display, Formatter};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Bool01 {
    False,
    True,
}

impl From<bool> for Bool01 {
    fn from(value: bool) -> Self {
        match value {
            false => Bool01::False,
            true => Bool01::True,
        }
    }
}

impl From<Bool01> for bool {
    fn from(value: Bool01) -> Self {
        match value {
            Bool01::False => false,
            Bool01::True => true,
        }
    }
}

impl From<&Bool01> for bool {
    fn from(value: &Bool01) -> Self {
        bool::from(*value)
    }
}

impl From<Bool01> for u8 {
    fn from(value: Bool01) -> Self {
        match value {
            Bool01::False => 0,
            Bool01::True => 1,
        }
    }
}

impl From<&Bool01> for u8 {
    fn from(value: &Bool01) -> Self {
        u8::from(*value)
    }
}

impl TryFrom<u8> for Bool01 {
    type Error = InvalidRawBool;
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Bool01::False),
            1 => Ok(Bool01::True),
            _ => Err(InvalidRawBool),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InvalidRawBool;

impl Display for InvalidRawBool {
   fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("invalid raw value for bool, expected 0 or 1")
   } 
}

impl std::error::Error for InvalidRawBool {}

impl Serialize for Bool01 {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(self.into())
    }
}

impl<'de> Deserialize<'de> for Bool01 {
    fn deserialize<D>(deserializer: D) -> Result<Bool01, D::Error>
    where
        D: Deserializer<'de>,
    {
        u8::deserialize(deserializer).and_then(|value| Bool01::try_from(value).map_err(de::Error::custom))
    }
}