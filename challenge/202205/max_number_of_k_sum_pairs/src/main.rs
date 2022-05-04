fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
    use std::collections::HashMap;

    let mut h: HashMap<i32, i32> = HashMap::new();
    for num in nums.iter() {
        *h.entry(*num).or_insert(0) += 1;
    }

    let mut ret = 0;
    loop {
        let mut updated = false;
        for num in nums.iter() {
            let diff = k - *num;
            let mut ok = false;
            if let Some(&v) = h.get(&num) {
                if let Some(&w) = h.get(&diff) {
                    if (*num == diff && w >= 2) || (*num != diff && (v >= 1 && w >= 1)) {
                        ok = true;
                    }
                }
            }

            if ok {
                *h.get_mut(num).unwrap() -= 1;
                *h.get_mut(&diff).unwrap() -= 1;

                ret += 1;
                updated = true;
            }
        }

        if !updated {
            break;
        }
    }

    ret
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let ret = max_operations(nums, 5);
    println!("ret={ret}");
}

#[test]
fn test_max_operations() {
    {
        let nums = vec![1, 2, 3, 4];
        let ret = max_operations(nums, 5);
        assert_eq!(ret, 2);
    }
    {
        let nums = vec![3, 1, 3, 4, 3];
        let ret = max_operations(nums, 6);
        assert_eq!(ret, 1);
    }
}
