fn find_kth_bit(n: i32, k: i32) -> char {
    let mut v = vec!['0'];
    for _ in 1..n {
        let w: Vec<char> = v
            .iter()
            .rev()
            .map(|c| if *c == '0' { '1' } else { '0' })
            .collect();
        v.push('1');
        v.extend(w);
    }

    v[(k - 1) as usize]
}

fn main() {
    println!("ret={}", find_kth_bit(4, 11));
}

#[test]
fn test() {
    assert_eq!(find_kth_bit(3, 1), '0');
    assert_eq!(find_kth_bit(4, 11), '1');
}
