use std::error::Error;
use core::fmt;

#[derive(Debug)]
pub enum TopologicalError {
    Custom,
    RepeatedNodes,
    InvalidTopologicalAssumptions,
    NotADag,
    FirstNodeHasIncomingEdges,
}

impl Error for TopologicalError {}

impl fmt::Display for TopologicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Custom => write!(f, "Custom error"),
            Self::RepeatedNodes => write!(f, "The list has repeated nodes."),
            Self::InvalidTopologicalAssumptions => write!(f, "Invalid topological assumptions."),
            Self::NotADag => write!(f, "Provided list does not conform to a DAG."),
            Self::FirstNodeHasIncomingEdges => write!(f, "List assumptions are not met, i.e. first node should not have incoming edges.")
        }
    }
}