fn title_to_number(column_title: String) -> i32 {
    column_title.bytes().fold(0, |acc, b| {
        (acc * 26) + (b - b'A' + 1) as i32
    })
}

fn main() {
    let ret = title_to_number("ZY".to_string());
    println!("ret={ret}");
}

#[test]
fn test_title_to_number() {
    assert_eq!(title_to_number("A".to_string()), 1);
    assert_eq!(title_to_number("AB".to_string()), 28);
    assert_eq!(title_to_number("ZY".to_string()), 701);
    assert_eq!(title_to_number("CFDGSXM".to_string()), 1000000001);
}
