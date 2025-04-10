fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
    let mut nums = nums;
    let mut ret = 0;
    loop {
        let len = nums.len();
        let mut ok = true;
        for i in 1..len {
            if nums[i - 1] > nums[i] {
                ok = false;
                break;
            }
        }

        if ok {
            break;
        }

        let mut min_sum = i32::MAX;
        let mut min_index = 0;
        for i in 0..len - 1 {
            let sum = nums[i] + nums[i + 1];
            if sum < min_sum {
                min_index = i;
                min_sum = sum;
            }
        }

        let mut tmp = vec![];
        let mut i = 0;
        while i < len {
            if i == min_index {
                tmp.push(min_sum);
                i += 2;
            } else {
                tmp.push(nums[i]);
                i += 1;
            }
        }

        nums = tmp;
        ret += 1;
    }

    ret
}

fn main() {
    let nums = vec![5, 2, 3, 1];
    let ret = minimum_pair_removal(nums);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let nums = vec![5, 2, 3, 1];
        let ret = minimum_pair_removal(nums);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![1, 2, 2];
        let ret = minimum_pair_removal(nums);
        assert_eq!(ret, 0);
    }
}
