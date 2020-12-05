use serde::{Serialize, Deserialize};
use std::fmt;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq)]
pub struct RevID {
    #[serde(rename="UUID")]
    value: uuid::Uuid,
}

pub const EMPTY: RevID = RevID { value: Uuid::nil() };

impl RevID {
    pub fn is_empty(&self) -> bool {
        self.value.is_nil()
    }
}

impl fmt::Display for RevID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value.to_simple_ref().to_string())
    }
}

pub fn new() -> RevID {
    RevID {
        value: Uuid::new_v4()
    }
}

pub fn parse(s: &String) -> RevID {
    let id = Uuid::parse_str(s).expect("Bad revision ID format!");
    RevID {
        value: id
    }
}

