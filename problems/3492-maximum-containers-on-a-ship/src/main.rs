fn max_containers(n: i32, w: i32, max_weight: i32) -> i32 {
    let max = n * n * w;
    if max_weight <= max {
        max_weight / w
    } else {
        n * n
    }
}

fn main() {
    let ret = max_containers(2, 3, 15);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_containers(2, 3, 15), 4);
    assert_eq!(max_containers(3, 5, 20), 4);
    assert_eq!(max_containers(2, 3, 4), 1);
}
