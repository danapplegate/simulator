use crate::output_adapter::OutputAdapter;
use crate::{Positioned, Simulation};

pub struct CsvAdapter<'a> {
    simulation: &'a Simulation<'a>,
}

impl<'a> OutputAdapter<'a> for CsvAdapter<'a> {
    fn new(simulation: &'a Simulation) -> Self {
        Self { simulation }
    }

    fn output(&self) {
        let mut headers = String::from("t,");
        for body in self.simulation.bodies() {
            headers.push_str(format!("{0}.x,{0}.y", body.label).as_str());
        }
        println!("{}", headers);
        for step in self.simulation.new_run() {
            print!("{:.1}", step.t);
            for (_, body) in step.body_states() {
                let position = body.position();
                println!(",{},{}", position.x, position.y);
            }
        }
    }
}
