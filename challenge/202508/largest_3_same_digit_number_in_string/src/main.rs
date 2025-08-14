fn largest_good_integer(num: String) -> String {
    let v: Vec<_> = num.bytes().map(|b| (b - b'0') as i32).collect();
    let max = v.windows(3).fold(-1, |acc, c| {
        if c[0] == c[1] && c[1] == c[2] {
            std::cmp::max(acc, c[0])
        } else {
            acc
        }
    });

    if max == -1 {
        "".to_string()
    } else {
       format!("{max}{max}{max}")
    }
}

fn main() {
    let ret = largest_good_integer("6777133339".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(largest_good_integer("6777133339".to_string()), "777");
    assert_eq!(largest_good_integer("2300019".to_string()), "000");
    assert_eq!(largest_good_integer("42352338".to_string()), "");
}
