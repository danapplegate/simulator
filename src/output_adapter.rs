use crate::simulation::Simulation;

pub trait OutputAdapter<'a, const N: usize> {
    fn new(simulation: &'a Simulation<N>) -> Self;
    fn output(&self);
}

pub mod csv_adapter;
pub mod stdout_adapter;
