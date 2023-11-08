fn is_reachable_at_time(sx: i32, sy: i32, fx: i32, fy: i32, t: i32) -> bool {
    let x_distance = (sx - fx).abs();
    let y_distance = (sy - fy).abs();

    if x_distance == 0 && y_distance == 0 && t == 1 {
        false
    } else {
        t >= std::cmp::max(x_distance, y_distance)
    }
}

fn main() {
    let ret = is_reachable_at_time(2, 4, 7, 7, 6);
    println!("ret={ret}");
}

#[test]
fn test_is_reachable_at_time() {
    assert!(is_reachable_at_time(2, 4, 7, 7, 6));
    assert!(!is_reachable_at_time(3, 1, 7, 3, 3));
    assert!(!is_reachable_at_time(0, 0, 0, 0, 1));
}
