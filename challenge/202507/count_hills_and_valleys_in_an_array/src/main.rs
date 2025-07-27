fn count_hill_valley(nums: Vec<i32>) -> i32 {
    let len = nums.len();

    let mut ret = 0;
    'outer: for i in 1..(len - 1) {
        if nums[i] == nums[i - 1] {
            continue;
        }

        let mut left = 0;
        for j in (0..i).rev() {
            if nums[j] != nums[i] {
                left = nums[j];
                break;
            }
            if j == 0 {
                continue 'outer;
            }
        }

        let mut right = 0;
        for j in (i + 1)..len {
            if nums[j] != nums[i] {
                right = nums[j];
                break;
            }
            if j == len - 1 {
                continue 'outer;
            }
        }

        if (left < nums[i] && nums[i] > right) || (left > nums[i] && nums[i] < right) {
            ret += 1;
        }
    }

    ret
}

fn main() {
    let nums = vec![2, 4, 1, 1, 6, 5];
    let ret = count_hill_valley(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![2, 4, 1, 1, 6, 5];
        let ret = count_hill_valley(nums);
        assert_eq!(ret, 3);
    }
    {
        let nums = vec![6, 6, 5, 5, 4, 1];
        let ret = count_hill_valley(nums);
        assert_eq!(ret, 0);
    }
}
