use simulator::{Body, Massive, PositionVector2, Simulation, SimulationType};

fn main() {
    let mut sim = Simulation::new(SimulationType::Newtonian, 2);
    let body = Body::new("body1", 5.0, Some(PositionVector2::new(0.0, 100.0)));
    println!("Body {} mass: {}g", body.label, body.mass());
    sim.add_body(body);
    println!("{:?}", sim);
    let run = sim.new_run();
    for step in run {
        println!("{:?}", step);
    }
}
