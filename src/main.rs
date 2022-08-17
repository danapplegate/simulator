use simulator::output_adapter::{OutputAdapter, StdOutAdapter};
use simulator::{Body, Massive, PositionVector2, Simulation};

fn main() {
    let mut sim = Simulation::new();
    let body = Body::new("body1", 5.0, Some(PositionVector2::new(0.0, 100.0)));
    println!("Body {} mass: {}g", body.label, body.mass());
    sim.add_body(body);
    println!("{:?}", sim);
    let output = StdOutAdapter::new(&sim);
    output.output();
}
