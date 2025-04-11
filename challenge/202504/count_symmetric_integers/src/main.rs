fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    (low..=high).fold(0, |acc, n| {
        let ns: Vec<_> = n.to_string().bytes().map(|b| (b - b'0') as i32).collect();
        let len = ns.len();
        if len % 2 != 0 {
            acc
        } else {
            let half = len / 2;
            let sum1: i32 = ns.iter().take(half).sum();
            let sum2: i32 = ns.iter().skip(half).sum();
            acc + (if sum1 == sum2 { 1 } else { 0 })
        }
    })
}

fn main() {
    let ret = count_symmetric_integers(1200, 1230);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_symmetric_integers(1, 100), 9);
    assert_eq!(count_symmetric_integers(1200, 1230), 4);
}