fn count_and_say(n: i32) -> String {
    fn to_run_length(s: &str) -> String {
        let mut count = 1;
        let mut ch = s.chars().next().unwrap();

        let mut v = vec![];
        for c in s.chars().skip(1) {
            if ch != c {
                v.push((ch, count));
                ch = c;
                count = 1;
            } else {
                count += 1;
            }
        }

        v.push((ch, count));

        let mut ret = String::new();
        for (c, count) in v {
            ret.push_str(&count.to_string());
            ret.push(c);
        }

        ret
    }

    let mut ret = "1".to_string();
    for _ in 1..n {
        ret = to_run_length(&ret);
    }

    ret
}

fn main() {
    let ret = count_and_say(20);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(count_and_say(4), "1211");
    assert_eq!(count_and_say(1), "1");
}
