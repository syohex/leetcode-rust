fn longest_balanced(s: String) -> i32 {
    let len = s.len();
    let cs: Vec<_> = s.chars().collect();

    let mut ret = 0;
    for i in 1..=len {
        for w in cs.windows(i) {
            let (freq, count) = w
                .iter()
                .fold((vec![0; 26], -1), |(mut acc, count), &v| {
                    let index = v as usize - 'a' as usize;
                    acc[index] += 1;
                    let tmp = acc[index];
                    (acc, std::cmp::max(count, tmp))
                });

            if freq.into_iter().filter(|&n| n != 0).all(|n| n == count) {
                ret = i as i32;
                break;
            }
        }
    }

    ret
}

fn main() {
    let ret = longest_balanced("zzabccy".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(longest_balanced("abbac".to_string()), 4);
    assert_eq!(longest_balanced("zzabccy".to_string()), 4);
    assert_eq!(longest_balanced("aba".to_string()), 2);
}
