fn max_distance(moves: String) -> i32 {
    let mut left_right = 0i32;
    let mut up_down = 0i32;
    let mut replaced = 0;

    for c in moves.chars() {
        match c {
            'U' => up_down += 1,
            'D' => up_down -= 1,
            'L' => left_right -= 1,
            'R' => left_right += 1,
            '_' => replaced += 1,
            _ => unreachable!("never reach here"),
        }
    }

    left_right.abs() + up_down.abs() + replaced
}

fn main() {
    let ret = max_distance("L_D_".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_distance("L_D_".to_string()), 4);
    assert_eq!(max_distance("U_R".to_string()), 3);
}
