fn robot_with_string(s: String) -> String {
    let mut counts = [0; 26];
    for b in s.bytes() {
        counts[(b - b'a') as usize] += 1;
    }

    let mut stack = vec![];
    let mut ret = String::new();
    let mut min_char = b'a';
    for b in s.bytes() {
        let index = (b - b'a') as usize;
        counts[index] -= 1;
        stack.push(b);

        while min_char != b'z' && counts[(min_char - b'a') as usize] == 0 {
            min_char += 1;
        }

        while !stack.is_empty() {
            let top = *stack.last().unwrap();
            if top > min_char {
                break;
            }

            stack.pop();
            ret.push(top as char);
        }
    }

    ret
}

fn main() {
    let ret = robot_with_string("aabbccaadd".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(robot_with_string("zza".to_string()), "azz");
    assert_eq!(robot_with_string("bac".to_string()), "abc");
    assert_eq!(robot_with_string("bdda".to_string()), "addb");
}
