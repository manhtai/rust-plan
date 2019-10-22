use std::str::from_utf8;

use serde::{Deserialize, Deserializer, Serialize, Serializer};
use serde::de::{self, Visitor};
use serde::export::fmt::Error;
use serde::export::Formatter;

pub struct RedisPing {
    pub message: String
}

struct RPVisitor;


impl Serialize for RedisPing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        let mut message = "PING ".to_owned();
        message.push_str(&self.message);
        serializer.serialize_bytes(message.as_ref())
    }
}

impl<'de> Visitor<'de> for RPVisitor {
    type Value = RedisPing;

    fn expecting<'a>(&self, formatter: &mut Formatter<'a>) -> Result<(), Error> {
        formatter.write_str("a message or nothing")
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E> where
        E: de::Error, {
        let (_, message) = from_utf8(v).unwrap_or("").split_at(4);
        Ok(RedisPing { message: message.to_string() })
    }
}


impl<'de> Deserialize<'de> for RedisPing {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where
        D: Deserializer<'de> {
        deserializer.deserialize_bytes(RPVisitor)
    }
}
