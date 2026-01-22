fn minimum_pair_removal(nums: Vec<i32>) -> i32 {
    let mut nums = nums;

    let mut ret = 0;
    loop {
        if nums.is_sorted() {
            return ret;
        }


        let len = nums.len();
        let mut min_sum = i32::MAX;
        let mut min_index = 0;

        for i in 1..len {
            let sum = nums[i - 1] + nums[i];
            if sum < min_sum {
                min_sum = sum;
                min_index = i - 1;
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
}

fn main() {
    let ret = minimum_pair_removal(vec![5,2,3,1]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(minimum_pair_removal(vec![3,6,4,-6,2,-4,5,-7,-3,6,3,-4]), 10);
    assert_eq!(minimum_pair_removal(vec![5,2,3,1]), 2);
    assert_eq!(minimum_pair_removal(vec![1,2,2]), 0);
}
