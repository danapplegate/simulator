use simulator::force::{Force, Gravity};
// use simulator::output_adapter::{csv_adapter::CsvAdapter, OutputAdapter};
use simulator::math::Distance;
use simulator::{Body, PositionVector2, Positioned};

// Earth mass in kg
const EARTH_MASS: f64 = 5.9722e+24;
// Earth radius in meters at equator
const EARTH_RADIUS: f64 = 6.3781370e+6;
// Earth radius in meters at pole
// const EARTH_RADIUS: f64 = 6.3567523e+6;

fn main() {
    let body1 = Body::new("body1", 1.0, Some(PositionVector2::new(EARTH_RADIUS, 0.0)));
    let body2 = Body::new("Earth", EARTH_MASS, Some(PositionVector2::new(0.0, 0.0)));
    println!(
        "Distance between {} and {}: {} m",
        body1.label,
        body2.label,
        body1.position().distance(&body2.position())
    );
    println!(
        "Direction to {} from {}: {:?}",
        body2.label,
        body1.label,
        body1.position().direction(&body2.position())
    );
    let mut gravity = Gravity::new();
    let force_vector = gravity.calculate(&body1, &body2);
    println!(
        "Force on {} by {}: {:?} N",
        body1.label, body2.label, force_vector
    );

    let fv2 = gravity.calculate(&body2, &body1);
    println!("Force on {} by {}: {:?} N", body2.label, body1.label, fv2);
}
