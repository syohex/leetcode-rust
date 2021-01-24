use std::cmp::max;

fn largest_altitude(gain: Vec<i32>) -> i32 {
    let mut ret = 0;
    let mut altitude = 0;
    for g in gain.iter() {
        altitude += g;
        ret = max(ret, altitude);
    }

    ret
}

fn main() {
    println!(
        "largest_altitude([-5, 1, 5, 0, -7]) = {}",
        largest_altitude(vec![-5, 1, 5, 0, -7])
    );
}

#[test]
fn test_largest_altitude() {
    assert_eq!(largest_altitude(vec![-5, 1, 5, 0, -7]), 1);
    assert_eq!(largest_altitude(vec![-4, -3, -2, -1, 4, 3, 2]), 0);
}
