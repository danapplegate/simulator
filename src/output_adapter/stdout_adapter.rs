use crate::output_adapter::OutputAdapter;
use crate::Simulation;

pub struct StdoutAdapter<'a> {
    simulation: &'a Simulation<'a>,
}

impl<'a> OutputAdapter<'a> for StdoutAdapter<'a> {
    fn new(simulation: &'a Simulation) -> Self {
        Self { simulation }
    }

    fn output(&self) {
        let run = self.simulation.new_run();
        for step in run {
            println!("{:.1} -> {:?}", step.t, step.body_states);
        }
    }
}
