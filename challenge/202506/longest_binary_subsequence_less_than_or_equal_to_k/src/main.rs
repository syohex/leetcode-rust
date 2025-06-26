fn longest_subsequence(s: String, k: i32) -> i32 {
    let mut ret = 0;
    let mut sum = 0;

    for (i, c) in s.chars().rev().enumerate() {
        let i = i as i32;
        if c == '1' {
            if i < 31 {
                let v = sum + (1 << i);
                if v <= k {
                    sum = v;
                    ret += 1;
                }
            }
        } else {
            ret += 1;
        }
    }

    ret
}
fn main() {
    let ret = longest_subsequence("00101001".to_string(), 1);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(longest_subsequence("1001010".to_string(), 5), 5);
    assert_eq!(longest_subsequence("00101001".to_string(), 1), 6);
    {
        let s = "111100010000011101001110001111000000001011101111111110111000011111011000010101110100110110001111001001011001010011010000011111101001101000000101101001110110000111101011000101".to_string();
        let k = 11713332;
        let ret = longest_subsequence(s, k);
        assert_eq!(ret, 96);
    }
    {
        let s = "100110111111000000010011101000111011000001000111010001010111100001111110110010100011100100111000011011000000100001011000000100110110001101011010011".to_string();
        let k = 522399436;
        let ret = longest_subsequence(s, k);
        assert_eq!(ret, 92);
    }
}
