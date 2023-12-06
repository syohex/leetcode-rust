fn total_money(n: i32) -> i32 {
    let mut ret = 0;
    let mut week = 0;

    for i in 0..n {
        ret += (i % 7 + 1) + week;
        if i % 7 == 6 {
            week += 1;
        }
    }

    ret
}

fn main() {
    let ret = total_money(20);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(total_money(4), 10);
    assert_eq!(total_money(10), 37);
    assert_eq!(total_money(20), 96);
}
