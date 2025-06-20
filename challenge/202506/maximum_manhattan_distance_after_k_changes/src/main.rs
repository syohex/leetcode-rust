fn max_distance(s: String, k: i32) -> i32 {
    let mut x = 0i32;
    let mut y = 0i32;

    let mut ret = 0;
    for (limit, c) in s.chars().enumerate() {
        match c {
            'N' => y += 1,
            'S' => y -= 1,
            'E' => x += 1,
            'W' => x -= 1,
            _ => unreachable!("never reach here"),
        }

        let distance = std::cmp::min(x.abs() + y.abs() + 2 * k, limit as i32 + 1);
        ret = std::cmp::max(ret, distance);
    }

    ret
}

fn main() {
    let ret = max_distance("NSWWEW".to_string(), 3);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_distance("NWSE".to_string(), 1), 3);
    assert_eq!(max_distance("NSWWEW".to_string(), 3), 6);
}
