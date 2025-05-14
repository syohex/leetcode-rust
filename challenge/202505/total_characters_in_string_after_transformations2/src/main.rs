fn length_after_transformation(s: String, t: i32, nums: Vec<i32>) -> i32 {
    let modulo = 1_000_000_007;
    let mut freq = [0; 26];

    for b in s.bytes() {
        let index = (b - b'a') as usize;
        freq[index] += 1;
    }

    for _ in 0..t {
        let mut tmp = [0; 26];
        for i in 0..26 {
            let count = freq[i];
            for j in 1..=nums[i] as usize {
                let index = (i + j) % 26;
                tmp[index] = (tmp[index] + count) % modulo;
            }
        }

        freq = tmp;
    }

    freq.into_iter().fold(0, |acc, n| (acc + n) % modulo)
}

fn main() {
    let s = "azbk".to_string();
    let t = 1;
    let nums = vec![
        2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
    ];
    let ret = length_after_transformation(s, t, nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let s = "abcyy".to_string();
        let t = 2;
        let nums = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2,
        ];
        let ret = length_after_transformation(s, t, nums);
        assert_eq!(ret, 7);
        {
            let s = "azbk".to_string();
            let t = 1;
            let nums = vec![
                2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            ];
            let ret = length_after_transformation(s, t, nums);
            assert_eq!(ret, 8);
        }
    }
}
