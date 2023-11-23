fn find_minimum_operations(s1: String, s2: String, s3: String) -> i32 {
    let mut s1 = s1;
    let mut s2 = s2;
    let mut s3 = s3;

    let mut ret = 0;
    let min_len = std::cmp::min(s1.len(), std::cmp::min(s2.len(), s3.len()));

    while s1.len() > min_len {
        s1.pop();
        ret += 1;
    }
    while s2.len() > min_len {
        s2.pop();
        ret += 1;
    }
    while s3.len() > min_len {
        s3.pop();
        ret += 1;
    }

    loop {
        if s1 == s2 && s2 == s3 {
            return ret;
        }

        ret += 3;
        s1.pop();
        s2.pop();
        s3.pop();

        if s1.is_empty() {
            return -1;
        }
    }
}

fn main() {
    let s1 = "abc".to_string();
    let s2 = "abb".to_string();
    let s3 = "ab".to_string();
    let ret = find_minimum_operations(s1, s2, s3);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s1 = "abc".to_string();
        let s2 = "abb".to_string();
        let s3 = "ab".to_string();
        let ret = find_minimum_operations(s1, s2, s3);
        assert_eq!(ret, 2);
    }
    {
        let s1 = "dac".to_string();
        let s2 = "bac".to_string();
        let s3 = "cac".to_string();
        let ret = find_minimum_operations(s1, s2, s3);
        assert_eq!(ret, -1);
    }
    {
        let s1 = "a".to_string();
        let s2 = "a".to_string();
        let s3 = "a".to_string();
        let ret = find_minimum_operations(s1, s2, s3);
        assert_eq!(ret, 0);
    }
    {
        let s1 = "ab".to_string();
        let s2 = "ac".to_string();
        let s3 = "ad".to_string();
        let ret = find_minimum_operations(s1, s2, s3);
        assert_eq!(ret, 3);
    }
    {
        let s1 = "abc".to_string();
        let s2 = "abc".to_string();
        let s3 = "abc".to_string();
        let ret = find_minimum_operations(s1, s2, s3);
        assert_eq!(ret, 0);
    }
}
