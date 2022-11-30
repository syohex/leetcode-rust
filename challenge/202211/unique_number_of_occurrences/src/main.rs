fn unique_occurrences(arr: Vec<i32>) -> bool {
    use std::collections::HashMap;
    use std::collections::HashSet;

    let mut freq = HashMap::new();
    for num in arr {
        *freq.entry(num).or_insert(0) += 1;
    }

    let mut s = HashSet::new();
    for v in freq.into_values() {
        if s.contains(&v) {
            return false;
        }

        s.insert(v);
    }

    true
}

fn main() {
    let nums = vec![1, 2, 2, 1, 1, 3];
    let ret = unique_occurrences(nums);
    println!("ret={ret}");
}

#[test]
fn test_unique_occurences() {
    {
        let nums = vec![1, 2, 2, 1, 1, 3];
        assert!(unique_occurrences(nums));
    }
    {
        let nums = vec![1, 2];
        assert!(!unique_occurrences(nums));
    }
    {
        let nums = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        assert!(unique_occurrences(nums));
    }
}
