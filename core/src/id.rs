use serde::{Deserialize, Serialize};
use snowflaked::sync::Generator;
use snowflaked::Snowflake;
use std::fmt::{Display, Formatter};

static GENERATOR: Generator = Generator::new(0);

macro_rules! id_struct {
    ($name:ident) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub u64);

        impl Snowflake for $name {
            fn from_parts(timestamp: u64, instance: u64, sequence: u64) -> Self {
                Self(u64::from_parts(timestamp, instance, sequence))
            }
            fn timestamp(&self) -> u64 {
                self.0.timestamp()
            }
            fn instance(&self) -> u64 {
                self.0.instance()
            }
            fn sequence(&self) -> u64 {
                self.0.sequence()
            }
        }
        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self(0)
            }
        }
        impl From<u64> for $name {
            fn from(id: u64) -> Self {
                Self(id)
            }
        }
        impl Into<u64> for $name {
            fn into(self) -> u64 {
                self.0
            }
        }
        impl $name {
            pub fn new() -> Self {
                Self::from(GENERATOR.generate::<$name>())
            }
            pub fn none() -> Self {
                Self(0)
            }
        }
    };
}

id_struct!(NodeID);
id_struct!(EdgeID);
