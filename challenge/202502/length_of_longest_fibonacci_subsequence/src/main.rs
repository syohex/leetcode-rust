fn len_longest_fib_subseq(arr: Vec<i32>) -> i32 {
    use std::collections::HashMap;

    let h = arr.iter().fold(HashMap::new(), |mut acc, &n| {
        *acc.entry(n).or_insert(0) += 1;
        acc
    });

    let mut ret = 0;
    let len = arr.len();
    for i in 0..len {
        for j in (i + 1)..len {
            if h.contains_key(&(arr[i] + arr[j])) {
                let mut count = 3;
                let (mut prev1, mut prev2) = (arr[i] + arr[j], arr[j]);
                while h.contains_key(&(prev1 + prev2)) {
                    (prev1, prev2) = (prev1 + prev2, prev1);
                    count += 1;
                }
                ret = std::cmp::max(ret, count);
            }
        }
    }

    ret
}
fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let ret = len_longest_fib_subseq(arr);
    println!("ret={ret}");
}

#[test]
fn test() {
    {
        let arr = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let ret = len_longest_fib_subseq(arr);
        assert_eq!(ret, 5);
    }
    {
        let arr = vec![1, 3, 7, 11, 12, 14, 18];
        let ret = len_longest_fib_subseq(arr);
        assert_eq!(ret, 3);
    }
}
