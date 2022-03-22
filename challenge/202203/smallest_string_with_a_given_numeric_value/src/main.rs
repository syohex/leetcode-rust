fn get_smallest_string(n: i32, k: i32) -> String {
    let mut v = vec![0u8; n as usize];
    let mut k = k;

    for i in (0..n).rev() {
        let m = std::cmp::min(k - i, 26);
        v[i as usize] = (m as u8) + b'a' - 1;
        k -= m as i32;
    }

    String::from_utf8(v).unwrap()
}

fn main() {
    let ret = get_smallest_string(5, 73);
    println!("ret={ret}");
}

#[test]
fn test_get_smallest_string() {
    assert_eq!(get_smallest_string(3, 27), "aay".to_string());
    assert_eq!(get_smallest_string(5, 73), "aaszz".to_string());
    assert_eq!(
        get_smallest_string(24, 552),
        "aadzzzzzzzzzzzzzzzzzzzzz".to_string()
    );
}
