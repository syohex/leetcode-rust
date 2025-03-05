fn colored_cells(n: i32) -> i64 {
    if n == 1 {
        1i64
    } else {
        let mut ret = 0i64;
        for i in 1..=n {
            ret += ((2 * i as i64) - 1) * 2;
        }
        ret - (2 * n as i64 - 1)
    }
}

fn main() {
    let ret = colored_cells(10000);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(colored_cells(1), 1);
    assert_eq!(colored_cells(2), 5);
    assert_eq!(colored_cells(3), 13);
    assert_eq!(colored_cells(4), 25);
}
