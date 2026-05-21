fn longest_common_prefix(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
    use std::collections::HashSet;

    let mut s = HashSet::new();
    for n in arr1 {
        let mut n = n;
        while !s.contains(&n) && n > 0 {
            s.insert(n);
            n /= 10;
        }
    }

    let mut ret = 0;
    for n in arr2 {
        let mut n = n;
        while !s.contains(&n) && n > 0 {
            n /= 10;
        }

        if n != 0 {
            ret = std::cmp::max(ret, n.ilog10() + 1);
        }
    }

    ret as i32
}

fn main() {
    let ret = longest_common_prefix(vec![1, 10, 100], vec![1000]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(longest_common_prefix(vec![1, 10, 100], vec![1000]), 3);
    assert_eq!(longest_common_prefix(vec![1, 2, 3], vec![4, 4, 4]), 0);
}
