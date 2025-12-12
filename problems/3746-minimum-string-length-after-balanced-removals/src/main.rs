fn min_length_after_removals(s: String) -> i32 {
    let (a, b) = s.chars().fold((0i32, 0i32), |(a, b), c| {
        if c == 'a' {
            (a + 1, b)
        } else {
            (a, b + 1)
        }
    });

    (a - b).abs()
}

fn main() {
    let ret = min_length_after_removals("aabbab".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(min_length_after_removals("aabbab".to_string()), 0);
    assert_eq!(min_length_after_removals("aaaa".to_string()), 4);
    assert_eq!(min_length_after_removals("aaabb".to_string()), 1);
}
