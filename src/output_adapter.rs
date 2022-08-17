use crate::Simulation;

pub trait OutputAdapter<'a> {
    fn new(simulation: &'a Simulation) -> Self;
    fn output(&self);
}

pub struct StdOutAdapter<'a> {
    simulation: &'a Simulation<'a>,
}

impl<'a> OutputAdapter<'a> for StdOutAdapter<'a> {
    fn new(simulation: &'a Simulation) -> Self {
        Self { simulation }
    }

    fn output(&self) {
        let run = self.simulation.new_run();
        for step in run {
            println!("{:?}", step);
        }
    }
}
