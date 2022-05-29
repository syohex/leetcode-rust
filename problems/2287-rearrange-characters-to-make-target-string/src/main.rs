fn rearrange_characters(s: String, target: String) -> i32 {
    fn f(s: &str) -> Vec<i32> {
        s.bytes().fold(vec![0; 26], |mut acc, b| {
            acc[(b - b'a') as usize] += 1;
            acc
        })
    }

    let mut s_table = f(&s);
    let t_table = f(&target);

    let mut ret = 0;
    loop {
        for i in 0..26 {
            s_table[i] -= t_table[i];
            if s_table[i] < 0 {
                return ret;
            }
        }

        ret += 1;
    }
}

fn main() {
    let s = "ilovecodingonleetcode".to_string();
    let target = "code".to_string();
    let ret = rearrange_characters(s, target);
    println!("ret={ret}");
}

#[test]
fn test_rearrange_characters() {
    {
        let s = "ilovecodingonleetcode".to_string();
        let target = "code".to_string();
        let ret = rearrange_characters(s, target);
        assert_eq!(ret, 2);
    }
    {
        let s = "abcba".to_string();
        let target = "abc".to_string();
        let ret = rearrange_characters(s, target);
        assert_eq!(ret, 1);
    }
    {
        let s = "abbaccaddaeea".to_string();
        let target = "aaaaa".to_string();
        let ret = rearrange_characters(s, target);
        assert_eq!(ret, 1);
    }
}
