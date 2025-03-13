fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
    fn f(nums: &[i32], queries: &[Vec<i32>], count: usize) -> bool {
        let mut steps = vec![0; nums.len() + 1];

        for q in queries.iter().take(count) {
            steps[q[0] as usize] += q[2];
            steps[(q[1] + 1) as usize] -= q[2];
        }

        let mut v = 0;
        for (i, &num) in nums.iter().enumerate() {
            v += steps[i];
            if v < num {
                return false;
            }
        }

        true
    }

    if !f(&nums, &queries, queries.len()) {
        return -1;
    }

    let mut left = 0 as i32;
    let mut right = queries.len() as i32 - 1;

    while left <= right {
        let mid = left + (right - left) / 2;
        if f(&nums, &queries, mid as usize) {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }

    left as i32
}

fn main() {
    let nums = vec![2, 0, 2];
    let queries = vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]];
    let ret = min_zero_array(nums, queries);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![0];
        let queries = vec![
            vec![0, 0, 2],
            vec![0, 0, 4],
            vec![0, 0, 4],
            vec![0, 0, 3],
            vec![0, 0, 5],
        ];
        let ret = min_zero_array(nums, queries);
        assert_eq!(ret, 0);
    }
    {
        let nums = vec![2, 0, 2];
        let queries = vec![vec![0, 2, 1], vec![0, 2, 1], vec![1, 1, 3]];
        let ret = min_zero_array(nums, queries);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![4, 3, 2, 1];
        let queries = vec![vec![1, 3, 2], vec![0, 2, 1]];
        let ret = min_zero_array(nums, queries);
        assert_eq!(ret, -1);
    }
}
