fn increasing_triplet(nums: Vec<i32>) -> bool {
    let mut first = std::i32::MAX;
    let mut second = std::i32::MAX;

    for num in nums {
        if num <= first {
            first = num;
        } else if num <= second {
            second = num;
        } else {
            return true;
        }
    }

    false
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let ret = increasing_triplet(nums);
    println!("ret={ret}");
}

#[test]
fn test_increasing_triplet() {
    {
        let nums = vec![1, 2, 3, 4, 5];
        assert!(increasing_triplet(nums));
    }
    {
        let nums = vec![5, 4, 3, 2, 1];
        assert!(!increasing_triplet(nums));
    }
    {
        let nums = vec![2, 1, 5, 0, 4, 6];
        assert!(increasing_triplet(nums));
    }
    {
        let nums = vec![1, 1, 1, 1, 1];
        assert!(!increasing_triplet(nums));
    }
}
