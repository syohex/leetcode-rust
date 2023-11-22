fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
    use std::collections::HashMap;

    let mut h = HashMap::new();
    for i in 0..nums.len() {
        for j in 0..nums[i].len() {
            h.entry(i + j).or_insert(vec![]).push(nums[i][j]);
        }
    }

    let mut ret = vec![];
    let mut i = 0;
    while let Some(v) = h.get(&i) {
        for n in v.iter().rev() {
            ret.push(*n);
        }
        i += 1;
    }

    ret
}
fn main() {
    let nums = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let ret = find_diagonal_order(nums);
    println!("ret={ret:?}");
}

#[test]
fn test() {
    {
        let nums = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 4, 2, 7, 5, 3, 8, 6, 9];
        let ret = find_diagonal_order(nums);
        assert_eq!(ret, expected);
    }
    {
        let nums = vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7],
            vec![8],
            vec![9, 10, 11],
            vec![12, 13, 14, 15, 16],
        ];
        let expected = vec![1, 6, 2, 8, 7, 3, 9, 4, 12, 10, 5, 13, 11, 14, 15, 16];
        let ret = find_diagonal_order(nums);
        assert_eq!(ret, expected);
    }
}
