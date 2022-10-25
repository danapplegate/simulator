use crate::output_adapter::OutputAdapter;
use crate::{compute_next_step, Body, BodyMap, Simulation};

pub struct CsvAdapter<'a, const N: usize> {
    simulation: &'a Simulation<N>,
}

impl<'a, const N: usize> OutputAdapter<'a, N> for CsvAdapter<'a, N> {
    fn new(simulation: &'a Simulation<N>) -> Self {
        Self { simulation }
    }

    fn output(&self) {
        println!("{}", self.headers());
        let mut body_map = self.simulation.create_body_map();
        let mut t = self.simulation.t_start();
        let t_step = self.simulation.t_step();
        let order = self
            .simulation
            .bodies()
            .iter()
            .map(|b| b.label.clone())
            .collect();
        while t <= self.simulation.t_end() {
            println!("{}", self.body_row(t, &body_map, &order));
            t += t_step;
            body_map = compute_next_step(body_map, t_step);
        }
    }
}

impl<'a, const N: usize> CsvAdapter<'a, N> {
    fn body_header(body: &Body<N>) -> String {
        let mut body_header = String::new();
        body_header.push_str(&format!("{}.1", body.label));
        for n in 2..=N {
            body_header.push_str(&format!(",{}.{}", body.label, n));
        }
        body_header
    }

    fn body_data(body_state: &'a Body<N>) -> String {
        let mut body_data = String::new();
        let position = body_state.position;
        for n in 0..N {
            body_data.push_str(&format!(",{}", position[n]));
        }
        body_data
    }

    fn headers(&self) -> String {
        let mut headers = String::from("t");
        for body in self.simulation.bodies() {
            headers.push_str(",");
            headers.push_str(&Self::body_header(body))
        }
        headers
    }

    fn body_row(&self, t: f64, body_states: &'a BodyMap<'a, N>, order: &Vec<String>) -> String {
        let mut row = format!("{:.1}", t);
        for label in order {
            if let Some(body) = body_states.get(label) {
                row.push_str(&format!("{}", &Self::body_data(body)));
            }
        }
        row
    }
}
