fn mirror_frequency(s: String) -> i32 {
    let mut cs = vec![0; 13];
    let mut ns = vec![0; 5];

    for b in s.bytes() {
        if b >= b'0' && b <= b'9' {
            let idx = (b - b'0') as usize;
            if b <= b'4' {
                ns[idx] += 1;
            } else {
                ns[9 - idx] -= 1;
            }
        } else {
            let idx = (b - b'a') as usize;
            if b <= b'm' {
                cs[idx] += 1;
            } else {
                cs[25 - idx] -= 1;
            }
        }
    }

    let v1 = cs.into_iter().fold(0, |acc, n: i32| acc + n.abs());
    let v2 = ns.into_iter().fold(0, |acc, n: i32| acc + n.abs());
    v1 + v2
}

fn main() {
    let ret = mirror_frequency("ab1z9".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(mirror_frequency("ab1z9".to_string()), 3);
    assert_eq!(mirror_frequency("4m7n".to_string()), 2);
    assert_eq!(mirror_frequency("byby".to_string()), 0);
}
