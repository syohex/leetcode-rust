fn count_symmetric_integers(low: i32, high: i32) -> i32 {
    (low..=high)
        .filter(|&n| {
            if n >= 1000 {
                let part1 = n / 1000 + (n % 1000) / 100;
                let part2 = (n % 100) / 10 + n % 10;
                part1 == part2
            } else if n >= 100 {
                false
            } else {
                n % 11 == 0
            }
        })
        .count() as i32
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
