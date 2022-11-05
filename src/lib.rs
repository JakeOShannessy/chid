use arrayvec::ArrayString;
#[cfg(feature = "serde")]
use serde::{
    de::{self, Visitor},
    Deserialize, Deserializer, Serialize, Serializer,
};
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Chid(ArrayString<50>);

impl Chid {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for Chid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ParseChidError {
    InvalidChar { position: usize, character: char },
    TooLong,
}

impl std::fmt::Display for ParseChidError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::InvalidChar {
                position,
                character,
            } => write!(
                f,
                "Invalid character {} at position {}",
                character, position
            ),
            Self::TooLong => write!(f, "CHID too long"),
        }
    }
}

impl std::error::Error for ParseChidError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::InvalidChar { .. } => None,
            Self::TooLong => None,
        }
    }
}

impl FromStr for Chid {
    type Err = ParseChidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let array_string = match ArrayString::<50>::from(s) {
            Ok(s) => s,
            Err(_) => return Err(ParseChidError::TooLong),
        };
        // Check that there are no invalid characters.
        if let Some(position) = array_string.find(|c: char| c == '.' || c == ' ') {
            return Err(ParseChidError::InvalidChar {
                position,
                character: array_string.chars().nth(position).unwrap(),
            });
        }
        Ok(Chid(array_string))
    }
}

#[cfg(feature = "serde")]
impl Serialize for Chid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}
#[cfg(feature = "serde")]
struct ChidVisitor;
#[cfg(feature = "serde")]
impl<'de> Visitor<'de> for ChidVisitor {
    type Value = Chid;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a run id beginning with \"scr-\"")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value.parse().map_err(de::Error::custom)
    }
}

#[cfg(feature = "serde")]
impl<'de> Deserialize<'de> for Chid {
    fn deserialize<D>(deserializer: D) -> Result<Chid, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ChidVisitor)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Title(ArrayString<256>);

impl Title {
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl std::fmt::Display for Title {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ParseTitleError {
    TooLong,
}

impl std::fmt::Display for ParseTitleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::TooLong => write!(f, "Title too long"),
        }
    }
}

impl std::error::Error for ParseTitleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Self::TooLong => None,
        }
    }
}

impl FromStr for Title {
    type Err = ParseTitleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let array_string = match ArrayString::<256>::from(s) {
            Ok(s) => s,
            Err(_) => return Err(ParseTitleError::TooLong),
        };
        Ok(Title(array_string))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_chid() {
        assert_eq!(
            Err(ParseChidError::TooLong),
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".parse::<Chid>()
        );
        assert_eq!("hello", "hello".parse::<Chid>().unwrap().as_str());
        assert_eq!(
            Err(ParseChidError::InvalidChar {
                position: 3,
                character: '.',
            }),
            "hel.lo".parse::<Chid>()
        );
    }
}
