fn count_and_say(n: i32) -> String {
    fn group_by(s: &str) -> Vec<(i32, char)> {
        let mut ret = vec![];
        let mut prev = s.chars().next().unwrap();
        let mut count = 1;
        for c in s.chars().skip(1) {
            if c == prev {
                count += 1;
            } else {
                ret.push((count, prev));
                prev = c;
                count = 1;
            }
        }

        ret.push((count, prev));
        ret
    }

    fn group_to_string(v: &Vec<(i32, char)>) -> String {
        let mut s = String::new();
        for (count, c) in v {
            s.push_str(&format!("{count}{c}"));
        }

        s
    }

    fn f(i: i32, n: i32, prev: String) -> String {
        if i == n {
            prev
        } else {
            let s = group_to_string(&group_by(&prev));
            f(i + 1, n, s)
        }
    }

    f(1, n, "1".to_string())
}

fn main() {
    let ret = count_and_say(4);
    println!("ret={ret}");
}

#[test]
fn test_count_and_say() {
    {
        let ret = count_and_say(1);
        assert_eq!(ret, "1");
    }
    {
        let ret = count_and_say(4);
        assert_eq!(ret, "1211");
    }
}
