use simulator::math::{Scalar, Vector1, Vector2};

fn main() {
    let first = Vector1(5.0);
    let second = Vector1(5.0);
    let third = Vector1(6.0);

    assert!(first == second);
    assert!(second != third);
    assert_eq!(first + second, Vector1(10.0));
    assert_eq!(third - second, Vector1(1.0));
    assert_eq!(5.0 * first, Vector1(25.0));
    assert_eq!(Scalar(2.0) * second, Vector1(10.0));
    println!("{:?}", first);

    let first_i32 = Vector1(-4);
    let second_i32 = Vector1(9);
    assert!(first_i32 != second_i32);
    assert_eq!(first_i32 + second_i32, Vector1(5));

    let first_2 = Vector2(1.0, 2.0);
    let second_2 = Vector2(1.0, 2.0);
    let third_2 = Vector2(3.0, 4.0);

    assert!(first_2 == second_2);
    assert!(second_2 != third_2);
    println!("{:?}", first_2);
}
