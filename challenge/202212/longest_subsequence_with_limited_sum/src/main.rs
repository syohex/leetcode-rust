pub fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort_unstable();

    let mut acc = vec![0; nums.len()];
    let mut sum = 0;
    for i in 0..nums.len() {
        acc[i] = nums[i] + sum;
        sum += nums[i];
    }

    let mut ret = vec![0; queries.len()];
    for (i, query) in queries.into_iter().enumerate() {
        for (j, a) in acc.iter().enumerate() {
            if *a <= query {
                ret[i] = (j + 1) as i32;
            } else {
                break;
            }
        }
    }

    ret
}

fn main() {
    let nums = vec![4, 5, 2, 1];
    let queries = vec![3, 10, 21];
    let ret = answer_queries(nums, queries);
    println!("ret={:?}", ret);
}

#[test]
fn test_answer_queries() {
    {
        let nums = vec![4, 5, 2, 1];
        let queries = vec![3, 10, 21];
        let expected = vec![2, 3, 4];
        let ret = answer_queries(nums, queries);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![2, 3, 4, 5];
        let queries = vec![1];
        let expected = vec![0];
        let ret = answer_queries(nums, queries);
        assert_eq!(ret, expected);
    }
}
