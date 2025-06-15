fn max_diff(num: i32) -> i32 {
    fn replace(bs: &mut [u8], b: u8, replaced: u8) {
        for a in bs {
            if *a == b {
                *a = replaced;
            }
        }
    }

    fn to_num(bs: &[u8]) -> i32 {
        bs.iter().fold(0, |acc, &b| acc * 10 + (b - b'0') as i32)
    }

    let s = num.to_string();
    let len = s.len();
    let mut max_num: Vec<_> = s.bytes().collect();

    for i in 0..len {
        let b = max_num[i];
        if b != b'9' {
            replace(&mut max_num, b, b'9');
            break;
        }
    }

    let mut min_num: Vec<_> = s.bytes().collect();
    for i in 0..len {
        let b = min_num[i];
        if i == 0 {
            if b != b'1' {
                replace(&mut min_num, b, b'1');
                break;
            }
        } else if b != min_num[0] && b != b'0' {
            replace(&mut min_num, b, b'0');
            break;
        }
    }

    to_num(&max_num) - to_num(&min_num)
}

fn main() {
    let ret = max_diff(555);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(max_diff(123456), 820000);
    assert_eq!(max_diff(555), 888);
    assert_eq!(max_diff(9), 8);
}
