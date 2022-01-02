fn num_pairs_divisible_by60(time: Vec<i32>) -> i32 {
    let mut dp = vec![0; 60];
    let mut ret = 0;
    for t in &time {
        let modulo = (*t % 60) as usize;
        ret += match modulo {
            0 => dp[0],
            n => dp[60 - n],
        };

        dp[modulo] += 1;
    }

    ret
}

fn main() {
    let time = vec![30, 20, 150, 100, 40];
    let ret = num_pairs_divisible_by60(time);
    println!("ret={}", ret);
}

#[test]
fn test_num_pairs_divisible_by60() {
    {
        let time = vec![30, 20, 150, 100, 40];
        assert_eq!(num_pairs_divisible_by60(time), 3);
    }
    {
        let time = vec![60, 60, 60];
        assert_eq!(num_pairs_divisible_by60(time), 3);
    }
}
