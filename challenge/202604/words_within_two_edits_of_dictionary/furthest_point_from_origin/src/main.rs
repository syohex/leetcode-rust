fn furthest_distance_from_origin(moves: String) -> i32 {
    let (ls, rs, us) = moves.chars().fold((0i32, 0, 0), |(ls, rs, us), c| match c {
        'L' => (ls + 1, rs, us),
        'R' => (ls, rs + 1, us),
        _ => (ls, rs, us + 1),
    });

    (ls - rs).abs() + us
}
fn main() {
    let ret = furthest_distance_from_origin("L_RL__R".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(furthest_distance_from_origin("L_RL__R".to_string()), 3);
    assert_eq!(furthest_distance_from_origin("_R__LL_".to_string()), 5);
    assert_eq!(furthest_distance_from_origin("_______".to_string()), 7);
}
