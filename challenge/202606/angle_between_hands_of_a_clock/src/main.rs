fn angle_clock(hour: i32, minutes: i32) -> f64 {
    let (hour, minutes) = (if hour == 12 { 0 } else { hour } as f64, minutes as f64);
    let hour_angle = 30.0 * (hour + minutes / 60.0);
    let minute_angle = 6.0 * minutes;
    let v = (hour_angle - minute_angle).abs();
    if v <= 180.0 { v } else { 360.0 - v }
}

fn main() {
    let ret = angle_clock(12, 30);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(angle_clock(12, 30), 165.0);
    assert_eq!(angle_clock(3, 30), 75.0);
    assert_eq!(angle_clock(3, 15), 7.5);
}
