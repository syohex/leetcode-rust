fn sort_permutation(nums: Vec<i32>) -> i32 {
    let (k, count) = nums
        .into_iter()
        .enumerate()
        .fold((0, 0), |(k, count), (i, n)| {
            if i != n as usize {
                if count == 0 {
                    (n, 1)
                } else {
                    (k & n, count + 1)
                }
            } else {
                (k, count)
            }
        });

    if count == 0 { 0 } else { k }
}

fn main() {
    let ret = sort_permutation(vec![0, 3, 2, 1]);
    println!("ret={ret}");
}

#[test]
fn test() {
    assert_eq!(sort_permutation(vec![0, 3, 2, 1]), 1);
    assert_eq!(sort_permutation(vec![0, 1, 3, 2]), 2);
    assert_eq!(sort_permutation(vec![3, 2, 1, 0]), 0);
}
