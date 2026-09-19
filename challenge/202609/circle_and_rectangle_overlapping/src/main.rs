fn check_overlap(
    radius: i32,
    x_center: i32,
    y_center: i32,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
) -> bool {
    use std::cmp::min;

    let mut distance = 0;
    if (x_center < x1) || (x_center > x2) {
        distance += min((x_center - x1).pow(2), (x_center - x2).pow(2));
    }
    if (y_center < y1) || (y_center > y2) {
        distance += min((y_center - y1).pow(2), (y_center - y2).pow(2));
    }
    distance <= radius.pow(2)
}

fn main() {
    let ret = check_overlap(1, 0, 0, 1, -1, 3, 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert!(check_overlap(1, 0, 0, 1, -1, 3, 1));
    assert!(!check_overlap(1, 1, 1, 1, -3, 2, -1));
    assert!(check_overlap(1, 0, 0, -1, 0, 0, 1));
}
