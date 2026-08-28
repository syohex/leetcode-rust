fn find_disappeared_numbers(nums: Vec<i32>, lower: i32, upper: i32) -> Vec<Vec<i32>> {
    let mut nums = nums;
    nums.push(lower - 1);
    nums.push(upper + 1);

    nums.sort_unstable();

    let mut ret = vec![];
    let len = nums.len();
    for i in 0..(len - 1) {
        if nums[i] == nums[i + 1] || nums[i] + 1 == nums[i+1] {
            continue;
        }
        if nums[i] < lower - 1 {
            continue;
        }
        if nums[i] > upper {
            break;
        }

        let end = nums[i + 1] - 1;
        ret.push(vec![nums[i] + 1, end]);
    }

    ret
}

fn main() {
    let nums = vec![3, 9, 7];
    let lower = 1;
    let upper = 12;
    let ret = find_disappeared_numbers(nums, lower, upper);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![34, 707, 380, 223, 455];
        let lower = 456;
        let upper = 974;
        let expected = vec![vec![456, 706], vec![708, 974]];
        let ret = find_disappeared_numbers(nums, lower, upper);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![2, 3, 5];
        let lower = 2;
        let upper = 3;
        let ret = find_disappeared_numbers(nums, lower, upper);
        assert!(ret.is_empty());
    }
    {
        let nums = vec![3, 9, 7];
        let lower = 1;
        let upper = 12;
        let expected = vec![vec![1, 2], vec![4, 6], vec![8, 8], vec![10, 12]];
        let ret = find_disappeared_numbers(nums, lower, upper);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 1];
        let lower = 5;
        let upper = 7;
        let expected = vec![vec![5, 7]];
        let ret = find_disappeared_numbers(nums, lower, upper);
        assert_eq!(ret, expected);
    }
}
