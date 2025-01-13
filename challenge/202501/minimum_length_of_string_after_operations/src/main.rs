fn minimum_length(s: String) -> i32 {
    s.bytes()
        .fold(vec![0; 26], |mut acc, b| {
            acc[(b - b'a') as usize] += 1;
            acc
        })
        .into_iter()
        .fold(0, |acc, n| {
            if n == 0 {
                acc
            } else if n % 2 == 1 {
                acc + 1
            } else {
                acc + 2
            }
        })
}

fn main() {
    let s = "abaacbcbb".to_string();
    let ret = minimum_length(s);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "abaacbcbb".to_string();
        let ret = minimum_length(s);
        assert_eq!(ret, 5);
    }
    {
        let s = "aa".to_string();
        let ret = minimum_length(s);
        assert_eq!(ret, 2);
    }
}
