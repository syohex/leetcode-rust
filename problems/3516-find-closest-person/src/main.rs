fn find_closest(x: i32, y: i32, z: i32) -> i32 {
    use std::cmp::Ordering;

    match (x - z).abs().cmp(&(y - z).abs()) {
        Ordering::Less => 1,
        Ordering::Greater => 2,
        Ordering::Equal => 0,
    }
}

fn main() {
    let ret = find_closest(2, 7, 4);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(find_closest(2, 7, 4), 1);
    assert_eq!(find_closest(2, 5, 6), 2);
    assert_eq!(find_closest(1, 5, 3), 0);
}
