use simulator::force::{Force, Gravity};
// use simulator::output_adapter::{csv_adapter::CsvAdapter, OutputAdapter};
use simulator::math::Distance;
use simulator::{Body, PositionVector2, Positioned};

// Earth mass in kg
const EARTH_MASS: f64 = 5.9722e+24;
// Earth radius in meters at equator
const EARTH_RADIUS: f64 = 6.3781e+6;

fn main() {
    let body1 = Body::new(
        "body1",
        1.0,
        Some(PositionVector2 {
            x: 0.0,
            y: EARTH_RADIUS,
        }),
    );
    let body2 = Body::new(
        "body2",
        EARTH_MASS,
        Some(PositionVector2 { x: 0.0, y: 0.0 }),
    );
    println!(
        "Distance between {} and {}: {} m",
        body1.label,
        body2.label,
        body1.position().distance(body2.position())
    );
    println!(
        "Gravitational force between {} and {}: {} N",
        body1.label,
        body2.label,
        Gravity::calculate_between(&body1, &body2)
    );
}
