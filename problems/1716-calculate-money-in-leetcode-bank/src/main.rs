fn total_money(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        let m = i % 7;
        let week = (i - 1) / 7;
        if m == 0 {
            sum += 7 + week;
        } else {
            sum += m + week;
        }
    }

    sum
}

fn main() {
    println!("total_money(20)={}", total_money(20));
}

#[test]
fn test_total_money() {
    assert_eq!(total_money(4), 10);
    assert_eq!(total_money(10), 37);
    assert_eq!(total_money(20), 96);
}
