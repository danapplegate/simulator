use crate::output_adapter::OutputAdapter;
use crate::simulation::{Run, Simulation};

pub struct StdoutAdapter<'a, const N: usize> {
    simulation: &'a Simulation<N>,
}

impl<'a, const N: usize> OutputAdapter<'a, N> for StdoutAdapter<'a, N> {
    fn new(simulation: &'a Simulation<N>) -> Self {
        Self { simulation }
    }

    fn output(&self) {
        let run = Run::from(self.simulation);
        for step in run {
            println!("{}: {:?}", step.t, step.body_map);
        }
    }
}
