fn find_closest_number(nums: Vec<i32>) -> i32 {
    nums.into_iter().fold(std::i32::MAX, |acc, n| {
        if n.abs() < acc.abs() || (n.abs() == acc.abs() && n > acc) {
            n
        } else {
            acc
        }
    })
}

fn main() {
    let nums = vec![-4, -2, 1, 4, 8];
    let ret = find_closest_number(nums);
    println!("ret={ret}");
}

#[test]
fn test_find_closest_number() {
    {
        let nums = vec![-4, -2, 1, 4, 8];
        let ret = find_closest_number(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![2, -1, 1];
        let ret = find_closest_number(nums);
        assert_eq!(ret, 1);
    }
    {
        let nums = vec![0];
        let ret = find_closest_number(nums);
        assert_eq!(ret, 0);
    }
}
