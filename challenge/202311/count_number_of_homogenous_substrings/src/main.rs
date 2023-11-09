fn count_homogenous(s: String) -> i32 {
    let cs: Vec<char> = s.chars().collect();
    let modulo = 1_000_000_007;

    let mut count = 1;
    let mut ret = 1;
    for i in 1..cs.len() {
        if cs[i] == cs[i - 1] {
            count += 1;
        } else {
            count = 1;
        }

        ret = (ret + count) % modulo;
    }

    ret
}

fn main() {
    let s = "abbcccaa".to_string();
    let ret = count_homogenous(s);
    println!("ret={ret}");
}

#[test]
fn test_count_homogenous() {
    {
        let s = "abbcccaa".to_string();
        let ret = count_homogenous(s);
        assert_eq!(ret, 13);
    }
    {
        let s = "xy".to_string();
        let ret = count_homogenous(s);
        assert_eq!(ret, 2);
    }
    {
        let s = "zzzzz".to_string();
        let ret = count_homogenous(s);
        assert_eq!(ret, 15);
    }
}
