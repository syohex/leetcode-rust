fn count_triples(n: i32) -> i32 {
    let mut ret = 0;
    for a in 1..=n {
        let aa = a * a;
        for b in a..=n {
            let bb = b * b;
            for c in b..=n {
                if aa + bb == c * c {
                    ret += 2;
                }
            }
        }
    }
    ret
}

fn main() {
    let ret = count_triples(10);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_triples(5), 2);
    assert_eq!(count_triples(10), 4);
}
