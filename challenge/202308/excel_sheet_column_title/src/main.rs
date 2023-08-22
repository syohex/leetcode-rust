fn convert_to_title(column_number: i32) -> String {
    let mut n = column_number;
    let mut ret = vec![];

    while n > 0 {
        n -= 1;

        ret.push(((n % 26) as u8 + b'A') as char);
        n /= 26;
    }

    ret.into_iter().rev().collect()
}

fn main() {
    println!("ret={}", convert_to_title(701));
}

#[test]
fn test_convert_to_title() {
    assert_eq!(convert_to_title(1), "A");
    assert_eq!(convert_to_title(28), "AB");
    assert_eq!(convert_to_title(701), "ZY");
}
