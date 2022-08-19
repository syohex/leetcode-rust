fn is_possible(nums: Vec<i32>) -> bool {
    use std::collections::HashMap;

    let mut h = nums.iter().fold(HashMap::new(), |mut acc, num| {
        *acc.entry(*num).or_insert(0) += 1;
        acc
    });

    let mut seqs: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        if let Some(v) = h.get_mut(&num) {
            if *v == 0 {
                continue;
            }
            *v -= 1;

            if let Some(w) = seqs.get_mut(&(num - 1)) {
                if *w >= 1 {
                    *w -= 1;
                    *seqs.entry(num).or_insert(0) += 1;
                    continue;
                }
            }

            if h.contains_key(&(num + 1)) && h.contains_key(&(num + 2)) {
                let next1 = *h.get(&(num + 1)).unwrap();
                let next2 = *h.get(&(num + 2)).unwrap();
                if next1 > 0 && next2 > 0 {
                    *seqs.entry(num + 2).or_insert(0) += 1;
                    h.insert(num + 1, next1 - 1);
                    h.insert(num + 2, next2 - 1);
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    true
}

fn main() {
    let nums = vec![1, 2, 3, 3, 4, 4, 5, 5];
    let ret = is_possible(nums);
    println!("ret={ret}");
}

#[test]
fn test_is_possible() {
    {
        let nums = vec![1, 2, 3, 3, 4, 5];
        let ret = is_possible(nums);
        assert!(ret);
    }
    {
        let nums = vec![1, 2, 3, 3, 4, 4, 5, 5];
        let ret = is_possible(nums);
        assert!(ret);
    }
    {
        let nums = vec![1, 2, 3, 4, 5, 6];
        let ret = is_possible(nums);
        assert!(ret);
    }
    {
        let nums = vec![1, 2, 3, 4, 4, 5];
        let ret = is_possible(nums);
        assert!(!ret);
    }
}
