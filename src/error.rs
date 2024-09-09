use std::error::Error;
use core::{
    fmt,
    num::ParseIntError,
};

#[derive(Debug)]
pub enum TopologicalError {
    Custom,
    RepeatedNodes,
    InvalidTopologicalAssumptions,
    NotADag,
    FirstNodeHasIncomingEdges,
    ParseIntError(ParseIntError),
}

impl Error for TopologicalError {}

impl fmt::Display for TopologicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom => write!(f, "Custom error"),
            Self::RepeatedNodes => write!(f, "The list has repeated nodes."),
            Self::InvalidTopologicalAssumptions => write!(f, "Invalid topological assumptions."),
            Self::NotADag => write!(f, "Provided list does not conform to a DAG."),
            Self::FirstNodeHasIncomingEdges => write!(f, "List assumptions are not met, i.e. first node should not have incoming edges."),
            Self::ParseIntError(e) => write!(f, "ParseIntError {e}"),
        }
    }
}

impl From<ParseIntError> for TopologicalError {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

#[test]
fn parse_int_error() {
        if let Err(error) = i32::from_str_radix("g12", 10) {
            let topological_error : TopologicalError = error.into();
        };
}