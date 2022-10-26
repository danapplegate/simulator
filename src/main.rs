use simulator::force::{Force, Gravity};
use simulator::math::Distance;
use simulator::output_adapter::{csv_adapter::CsvAdapter, OutputAdapter};
use simulator::{Body, PositionVector, Simulation, VelocityVector};

/// Earth mass in kg
const EARTH_MASS: f64 = 5.9722e+24;
/// Earth radius in meters at equator
const EARTH_RADIUS: f64 = 6.3781370e+6;
const LEO_RADIUS: f64 = EARTH_RADIUS + 2_000_000.0;
const LEO_SPEED: f64 = 6_900.0;

/// 2h 8m
const LEO_PERIOD: f64 = 2.0 * 3600.0 + 8.0 * 60.0;

/// Earth radius in meters at pole
// const EARTH_RADIUS: f64 = 6.3567523e+6;

fn main() {
    let body1 = Body::new(
        String::from("body1"),
        1.0,
        Some(PositionVector::from([0.0, LEO_RADIUS])),
        Some(VelocityVector::from([LEO_SPEED, 0.0])),
    );
    let body2 = Body::new(
        String::from("Earth"),
        EARTH_MASS,
        Some(PositionVector::from([0.0, 0.0])),
        None,
    );
    println!(
        "Distance between {} and {}: {} m",
        body1.label,
        body2.label,
        body1.position.distance(&body2.position)
    );
    println!(
        "Direction to {} from {}: {:?}",
        body2.label,
        body1.label,
        body1.position.direction(&body2.position)
    );
    let gravity = Gravity::new(None);
    let force_vector = gravity.calculate(&body1, &body2);
    println!(
        "Force on {} by {}: {:?} N",
        body1.label, body2.label, force_vector
    );

    let fv2 = gravity.calculate(&body2, &body1);
    println!("Force on {} by {}: {:?} N", body2.label, body1.label, fv2);

    let mut sim = Simulation::new(Some(0.0), Some(LEO_PERIOD), Some(0.1));
    sim.add_body(body1);
    sim.add_body(body2);

    let csv = CsvAdapter::new(&sim);
    csv.output();
}
