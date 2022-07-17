fn number_of_pairs(nums: Vec<i32>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut h: HashMap<i32, i32> = HashMap::new();
    for num in nums {
        *h.entry(num).or_insert(0) += 1;
    }

    let mut ret = vec![0; 2];
    for (_, v) in h.into_iter() {
        ret[0] += v / 2;
        ret[1] += v % 2;
    }

    ret
}

fn main() {
    let nums = vec![1, 3, 2, 1, 3, 2, 2];
    let ret = number_of_pairs(nums);
    println!("ret={:?}", ret);
}

#[test]
fn test_number_of_pairs() {
    {
        let nums = vec![1, 3, 2, 1, 3, 2, 2];
        let expected = vec![3, 1];
        let ret = number_of_pairs(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![1, 1];
        let expected = vec![1, 0];
        let ret = number_of_pairs(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![0];
        let expected = vec![0, 1];
        let ret = number_of_pairs(nums);
        assert_eq!(ret, expected);
    }
}
