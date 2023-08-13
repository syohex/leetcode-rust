fn valid_partion(nums: Vec<i32>) -> bool {
    use std::collections::HashMap;

    fn f(i: i32, nums: &Vec<i32>, cache: &mut HashMap<i32, bool>) -> bool {
        if i < 0 {
            return true;
        }

        if let Some(r) = cache.get(&i) {
            return *r;
        }

        let mut ret = false;
        if i >= 1 && nums[i as usize - 1] == nums[i as usize] {
            ret |= f(i - 2, nums, cache);
        }
        if i >= 2
            && nums[i as usize - 2] == nums[i as usize - 1]
            && nums[i as usize - 1] == nums[i as usize]
        {
            ret |= f(i - 3, nums, cache);
        }
        if i >= 2
            && nums[i as usize - 2] == nums[i as usize - 1] - 1
            && nums[i as usize - 1] == nums[i as usize] - 1
        {
            ret |= f(i - 3, nums, cache);
        }

        cache.insert(i, ret);
        ret
    }

    let mut cache = HashMap::new();
    f(nums.len() as i32 - 1, &nums, &mut cache)
}

fn main() {
    let nums = vec![4, 4, 4, 5, 6];
    let ret = valid_partion(nums);
    println!("ret={ret}");
}

#[test]
fn test_valid_partiton() {
    {
        let nums = vec![4, 4, 4, 5, 6];
        let ret = valid_partion(nums);
        assert!(ret);
    }
    {
        let nums = vec![1, 1, 1, 2];
        let ret = valid_partion(nums);
        assert!(!ret);
    }
}
