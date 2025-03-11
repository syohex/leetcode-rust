fn number_of_substrings(s: String) -> i32 {
    let cs: Vec<_> = s.bytes().collect();
    let mut left = 0;
    let mut right = 0;

    let mut ret = 0;
    let len = cs.len();
    let mut abc = [0; 3];
    while right < len {
        let index = (cs[right] - b'a') as usize;
        abc[index] += 1;

        while abc.iter().all(|&v| v >= 1) {
            ret += len - right;

            let index = (cs[left] - b'a') as usize;
            abc[index] -= 1;
            left += 1;
        }

        right += 1;
    }

    ret as i32
}

fn main() {
    let ret = number_of_substrings("abcabc".to_string());
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "abcabc".to_string();
        let ret = number_of_substrings(s);
        assert_eq!(ret, 10);
    }
    {
        let s = "aaacb".to_string();
        let ret = number_of_substrings(s);
        assert_eq!(ret, 3);
    }
    {
        let s = "abc".to_string();
        let ret = number_of_substrings(s);
        assert_eq!(ret, 1);
    }
}
