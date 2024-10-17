fn maximum_swap(num: i32) -> i32 {
    let s = num.to_string();
    let mut bs: Vec<u8> = s.bytes().collect();
    let len = bs.len();

    for i in 0..len {
        let mut max = bs[i];
        let mut pos = len;
        for j in (i + 1)..len {
            if max < bs[j] || (pos != len && max <= bs[j]) {
                max = bs[j];
                pos = j;
            }
        }

        if pos != len {
            bs.swap(i, pos);
            break;
        }
    }

    let mut ret = 0;
    for b in bs {
        ret = 10 * ret + (b - b'0') as i32;
    }
    ret
}

fn main() {
    let ret = maximum_swap(1993);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(maximum_swap(2736), 7236);
    assert_eq!(maximum_swap(9973), 9973);
    assert_eq!(maximum_swap(1993), 9913);
    assert_eq!(maximum_swap(22341345), 52341342);
    assert_eq!(maximum_swap(98368), 98863);
}
