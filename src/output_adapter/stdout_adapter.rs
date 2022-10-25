use crate::output_adapter::OutputAdapter;
use crate::{compute_next_step, BodyMap, Simulation};

pub struct StdoutAdapter<'a, const N: usize> {
    simulation: &'a Simulation<N>,
}

impl<'a, const N: usize> OutputAdapter<'a, N> for StdoutAdapter<'a, N> {
    fn new(simulation: &'a Simulation<N>) -> Self {
        Self { simulation }
    }

    fn output(&self) {
        let mut body_map = self.simulation.create_body_map();
        let mut t = self.simulation.t_start();
        let t_step = self.simulation.t_step();
        while t <= self.simulation.t_end() {
            println!("{}: {:?}", t, body_map);
            t += t_step;
            body_map = compute_next_step(body_map, t_step);
        }
    }
}
