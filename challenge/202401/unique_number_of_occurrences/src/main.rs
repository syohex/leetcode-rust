fn unique_occurrences(arr: Vec<i32>) -> bool {
    use std::collections::HashMap;
    use std::collections::HashSet;

    let mut freq = HashMap::new();
    for n in arr {
        *freq.entry(n).or_insert(0) += 1;
    }

    let mut vs = HashSet::new();
    for n in freq.values() {
        if vs.contains(n) {
            return false;
        }

        vs.insert(*n);
    }

    true
}

fn main() {
    let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
    let ret = unique_occurrences(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![1, 2, 2, 1, 1, 3];
        assert!(unique_occurrences(arr));
    }
    {
        let arr = vec![1, 2];
        assert!(!unique_occurrences(arr));
    }
    {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        assert!(unique_occurrences(arr));
    }
}
