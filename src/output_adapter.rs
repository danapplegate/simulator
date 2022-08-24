use crate::Simulation;

pub trait OutputAdapter<'a> {
    fn new(simulation: &'a Simulation) -> Self;
    fn output(&self);
}

pub mod csv_adapter;
pub mod stdout_adapter;
