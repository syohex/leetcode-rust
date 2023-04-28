fn find_prefix_score(nums: Vec<i32>) -> Vec<i64> {
    let mut max = 0i64;
    let mut acc = 0i64;

    let mut ret: Vec<i64> = vec![];

    for num in nums {
        let num = num as i64;

        max = std::cmp::max(max, num);
        acc += max + num;

        ret.push(acc);
    }

    ret
}

fn main() {
    let nums = vec![2, 3, 7, 5, 10];
    let ret = find_prefix_score(nums);
    println!("ret={ret:?}");
}

#[test]
fn test_find_prefix_score() {
    {
        let nums = vec![2, 3, 7, 5, 10];
        let expected = vec![4i64, 10, 24, 36, 56];
        let ret = find_prefix_score(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 1, 2, 4, 8, 16];
        let expected = vec![2, 4, 8, 16, 32, 64i64];
        let ret = find_prefix_score(nums);
        assert_eq!(ret, expected);
    }
}
