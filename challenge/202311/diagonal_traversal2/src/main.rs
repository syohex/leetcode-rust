fn find_diagonal_order(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let rows = nums.len();
    let cols = nums.iter().fold(0, |acc, v| {
        std::cmp::max(acc, v.len())
    });

    let mut ret = vec![];
    for i in 0..rows {
        let mut i = i as i32;

        let mut j = 0;
        loop {
            if i < 0 {
                break;
            }

            if let Some(&v) = nums[i as usize].get(j) {
                ret.push(v);
            }

            i -= 1;
            j += 1;
        }
    }

    for mut j in 1..cols {
        let mut i = (rows - 1) as i32;
        loop {
            if i < 0 {
                break;
            }

            if let Some(&v) = nums[i as usize].get(j) {
                ret.push(v);
            }

            i -= 1;
            j += 1;
        }
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
