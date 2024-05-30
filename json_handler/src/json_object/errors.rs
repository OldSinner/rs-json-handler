use std::fmt::{self};

#[derive(Debug, Clone)]
pub struct JsonObjectError();

impl fmt::Display for JsonObjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error occured during json maniuplation")
    }
}
