fn number_of_points(nums: Vec<Vec<i32>>) -> i32 {
    let mut nums = nums;

    nums.sort_by(|a, b| {
        if a[0] == b[0] {
            b[1].cmp(&a[1])
        } else {
            a[0].cmp(&b[0])
        }
    });

    let mut ret = 0;
    let mut s = nums[0][0];
    let mut e = nums[0][1];

    for v in nums.into_iter().skip(1) {
        if v[0] > e {
            ret += e - s + 1;
            s = v[0];
            e = v[1];
        } else {
            s = std::cmp::min(s, v[0]);
            e = std::cmp::max(e, v[1]);
        }
    }

    ret += e - s + 1;

    ret
}

fn main() {
    let nums = vec![vec![1, 3], vec![5, 8]];
    let ret = number_of_points(nums);
    println!("ret={ret}");
}

#[test]
fn test_number_of_points() {
    {
        let nums = vec![vec![3, 6], vec![1, 5], vec![4, 7]];
        let ret = number_of_points(nums);
        assert_eq!(ret, 7);
    }
    {
        let nums = vec![vec![1, 3], vec![5, 8]];
        let ret = number_of_points(nums);
        assert_eq!(ret, 7);
    }
    {
        let nums = vec![vec![1, 2], vec![3, 4], vec![5, 6]];
        let ret = number_of_points(nums);
        assert_eq!(ret, 6);
    }
    {
        let nums = vec![vec![1, 10], vec![3, 4], vec![5, 6]];
        let ret = number_of_points(nums);
        assert_eq!(ret, 10);
    }
}
