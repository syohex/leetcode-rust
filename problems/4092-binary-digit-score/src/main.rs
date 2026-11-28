fn calculate_score(a: i32, b: i32) -> i32 {
    fn count_ones(mut n: i32) -> i32 {
        let mut ret = 0;
        while n > 0 {
            ret += n % 2;
            n /= 2;
        }
        ret
    }

    let s1 = count_ones(a);
    let s2 = count_ones(b);

    if s1 >= s2 { s1 * s2 } else { s2 / s1 }
}

fn main() {
    let ret = calculate_score(14, 6);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(calculate_score(14, 6), 6);
    assert_eq!(calculate_score(5, 15), 2);
    assert_eq!(calculate_score(10, 12), 4);
}
