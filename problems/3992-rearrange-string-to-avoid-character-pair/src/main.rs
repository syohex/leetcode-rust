fn rearrange_string(s: String, x: char, _y: char) -> String {
    let mut x_count = 0;
    let mut ret = String::new();

    for c in s.chars() {
        if c == x {
            x_count += 1;
        } else {
            ret.push(c);
        }
    }

    for _ in 0..x_count {
        ret.push(x);
    }

    ret
}

fn main() {
    let ret = rearrange_string("aabc".to_string(), 'a', 'c');
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(rearrange_string("aabc".to_string(), 'a', 'c'), "bcaa");
    assert_eq!(rearrange_string("dcab".to_string(), 'd', 'b'), "cabd");
    assert_eq!(rearrange_string("axe".to_string(), 'o', 'x'), "axe");
}
