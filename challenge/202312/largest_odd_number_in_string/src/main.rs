fn largest_add_number(num: String) -> String {
    let bs: Vec<u8> = num.bytes().collect();
    for i in (0..bs.len()).into_iter().rev() {
        let n = bs[i] - b'0';
        if n % 2 == 1 {
            return std::str::from_utf8(&bs[0..=i]).unwrap().to_string();
        }
    }

    "".to_string()
}

fn main() {
    let ret = largest_add_number("35427".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(largest_add_number("52".to_string()), "5");
    assert_eq!(largest_add_number("4206".to_string()), "");
    assert_eq!(largest_add_number("35427".to_string()), "35427");
    assert_eq!(largest_add_number("7542351161".to_string()), "7542351161");
}
