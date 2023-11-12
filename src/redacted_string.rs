use std::fmt::{self, Debug, Display};

pub struct RedactedString(pub String);

impl Display for RedactedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "***")
    }
}

impl Debug for RedactedString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "RedactedString(***)")
    }
}
