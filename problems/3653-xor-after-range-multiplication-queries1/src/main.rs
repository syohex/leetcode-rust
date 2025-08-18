fn xor_after_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    let mut nums: Vec<_> = nums.into_iter().map(|n| n as i64).collect();
    let modulo = 1_000_000_007i64;

    for q in queries {
        let mut i = q[0] as usize;
        let end = q[1] as usize;

        while i <= end {
            nums[i] = (nums[i] * q[3] as i64) % modulo;
            i += q[2] as usize;
        }
    }

    nums.into_iter().fold(0, |acc, n| acc ^ n) as i32
}

fn main() {
    let nums = vec![2, 3, 1, 5, 4];
    let queries = vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]];
    let ret = xor_after_queries(nums, queries);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![780];
        let queries = vec![
            vec![0, 0, 1, 13],
            vec![0, 0, 1, 17],
            vec![0, 0, 1, 9],
            vec![0, 0, 1, 18],
            vec![0, 0, 1, 16],
            vec![0, 0, 1, 6],
            vec![0, 0, 1, 4],
            vec![0, 0, 1, 11],
            vec![0, 0, 1, 7],
            vec![0, 0, 1, 18],
            vec![0, 0, 1, 8],
            vec![0, 0, 1, 15],
            vec![0, 0, 1, 12],
        ];
        let ret = xor_after_queries(nums, queries);
        assert_eq!(ret, 523618060);
    }
    {
        let nums = vec![1, 1, 1];
        let queries = vec![vec![0, 2, 1, 4]];
        let ret = xor_after_queries(nums, queries);
        assert_eq!(ret, 4);
    }
    {
        let nums = vec![2, 3, 1, 5, 4];
        let queries = vec![vec![1, 4, 2, 3], vec![0, 2, 1, 2]];
        let ret = xor_after_queries(nums, queries);
        assert_eq!(ret, 31);
    }
}
