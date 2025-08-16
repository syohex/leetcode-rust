fn maximum69_number(num: i32) -> i32 {
    let mut ret = 0;
    let mut used = false;
    for b in num.to_string().bytes() {
        let v = if !used && b == b'6' {
            used = true;
            9
        } else {
            (b - b'0') as i32
        };

        ret = 10 * ret + v;
    }
    ret
}

fn main() {
    let ret = maximum69_number(666666);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(maximum69_number(9669), 9969);
    assert_eq!(maximum69_number(9996), 9999);
    assert_eq!(maximum69_number(9999), 9999);
}
