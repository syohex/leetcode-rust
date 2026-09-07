fn count_rotations(s: String, k: i32) -> i32 {
    let cs: Vec<_> = s.chars().collect();
    let len = cs.len();
    let mut ret = 0;

    for i in 0..len {
        let mut prev = i;
        let mut score = 0;
        for j in 1..len {
            let p = (i + j) % len;
            if cs[p] == cs[prev] {
                score += 1;
            }
            prev = p;
        }

        if score == k {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let ret = count_rotations("aab".to_string(), 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_rotations("aab".to_string(), 1), 2);
    assert_eq!(count_rotations("abca".to_string(), 0), 1);
}
