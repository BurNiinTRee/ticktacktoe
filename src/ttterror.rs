#[derive(Debug)]
pub enum TttError {
    InvalidField,
    NonPlayer,
    OccupiedField,
}

impl ::std::fmt::Display for TttError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        write!(f, "{}", ::std::error::Error::description(self))
    }
}

impl ::std::error::Error for TttError {
    fn description(&self) -> &str {
        match *self {
            TttError::InvalidField => "you're only allowed to enter 0 to 8",
            TttError::NonPlayer => "This should never happen!!!!",
            TttError::OccupiedField => "The field attempted to tick is already occupied!",
        }
    }
}


