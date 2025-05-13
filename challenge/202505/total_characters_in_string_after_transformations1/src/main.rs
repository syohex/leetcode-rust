fn length_after_transformations(s: String, t: i32) -> i32 {
    let modulo = 1_000_000_007;
    let mut table = [0; 26];
    for b in s.bytes() {
        let index = (b - b'a') as usize;
        table[index] += 1;
    }

    for _ in 0..t {
        let tmp0 = table[0];
        let tmp1 = table[1];
        table[0] = table[25];
        table[1] = (tmp0 + table[25]) % modulo;

        for i in (3..26).rev() {
            table[i] = table[i - 1];
        }
        table[2] = tmp1;
    }

    table.into_iter().fold(0, |acc, n| (acc + n) % modulo)
}
fn main() {
    let ret = length_after_transformations("abcyy".to_string(), 2);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let ret = length_after_transformations("abcyy".to_string(), 2);
        assert_eq!(ret, 7);
    }
    {
        let ret = length_after_transformations("azbk".to_string(), 1);
        assert_eq!(ret, 5);
    }
}
